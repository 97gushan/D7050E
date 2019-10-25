pub mod llvm_generator{


    use inkwell::OptimizationLevel;
    use inkwell::builder::Builder;
    use inkwell::context::Context;
    use inkwell::execution_engine::{ExecutionEngine, JitFunction};
    use inkwell::module::Module;
    use inkwell::IntPredicate;
    use inkwell::passes::PassManager;

    use inkwell::basic_block::BasicBlock;
    use inkwell::targets::{InitializationConfig, Target};
    use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, InstructionValue};
    use inkwell::types::BasicType;
    use inkwell::AddressSpace;
    
    use std::error::Error;
    use std::convert::TryInto;

    use std::collections::HashMap;


    use crate::ast::{ExprTree, BinOp, LogOp, NumCompOp};


    pub struct LLVM<'a>{
        context: &'a Context,
        module: &'a Module,
        builder: &'a Builder,
        variables: HashMap<String, PointerValue>,
        fn_value_opt: Option<FunctionValue>
    }

    impl<'a> LLVM<'a>{

        #[inline]
        fn get_function(&self, name: &str) -> Option<FunctionValue>{
            self.module.get_function(name)
        }

        #[inline]
        fn get_variable(&self, name: &str) -> &PointerValue{
            match self.variables.get(name) {
                Some(var) => var,
                None => panic!("ERROR: Can't find matching variable")
            }
        }

        fn fn_value(&self) -> FunctionValue{
            self.fn_value_opt.unwrap()
        }

        fn create_entry_block_alloca(&mut self, name: &str) -> PointerValue{
            let builder = self.context.create_builder();

            let entry = self.fn_value().get_first_basic_block().unwrap();

            match entry.get_first_instruction(){
                Some(instruction) => builder.position_before(&instruction),
                None => builder.position_at_end(&entry)
            }
            println!("wut");
            let alloca = builder.build_alloca(self.context.i32_type(), "a");
            println!("uuuuu");
            self.variables.insert(name.to_string(), alloca);
            alloca
        }

        fn match_node(&self, expr: &ExprTree) -> IntValue{
            match expr{
                ExprTree::Var(name) => {
                    let var = self.get_variable(&name);
                    self.builder.build_load(*var, &name).into_int_value()
                }
                ExprTree::Number(num) => self.compile_num(*num),
                
                ExprTree::Bool(b) => self.compile_bool(*b),
                ExprTree::BinNode(l, op, r) => self.compile_bin(*op, self.match_node(l), self.match_node(r)),
                ExprTree::NumCompNode(l, op, r) => self.compile_num_comp(*op, self.match_node(l), self.match_node(r)),
                ExprTree::LogNode(l, op, r) => self.compile_log(*op, self.match_node(l), self.match_node(r)),

                
                _ => panic!("")
            }
        }

        fn match_instruction(&mut self, expr: Box<ExprTree>) -> (InstructionValue, bool){
            match *expr.clone(){
                ExprTree::AssignNode(var, _t, val) => match *var{
                    ExprTree::Var(name) => {
                        let expr = self.match_node(&val);
                        let alloca = self.create_entry_block_alloca("a");
                        let store = self.builder.build_store(alloca, expr);
                        

                        (store, false)
                    },
                    _ => panic!("WWWWW")
                },
                ExprTree::SetVarNode(var, val) => match *var{
                    ExprTree::Var(name) => {
                        let var = self.get_variable(&name);
                        let expr = self.match_node(&val);
                        (self.builder.build_store(*var, expr), false)
                    },
                    _ => panic!("WWW"),
                },
                ExprTree::Return(expr) => {
                    (self.builder.build_return(Some(&self.match_node(&expr))), true)
                }
               
                _ => panic!("CHAOS")
            }
        }

        pub fn compile_block(&mut self, expr: Box<ExprTree>) -> InstructionValue{
            match *expr{
                ExprTree::SeqNode(b1, b2) => {
                    let (inst, ret) = self.match_instruction(b1);

                    if ret{
                        inst
                    }else{
                        self.compile_block(b2)
                    }
                }
                _ => {
                    let (inst, ret) = self.match_instruction(expr);

                    inst
                }
            }
        }

        fn compile_log(&self, op: LogOp, l: IntValue, r: IntValue) -> IntValue{
            match op{
                LogOp::And => self.builder.build_and(l, r, "and"),
                LogOp::Or => self.builder.build_or(l, r, "or"),

            }
        }

        fn compile_num_comp(&self, op: NumCompOp, l: IntValue, r: IntValue) -> IntValue{
            match op{
                NumCompOp::Eq => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                NumCompOp::Neq => self.builder.build_int_compare(IntPredicate::NE, l, r, "neq"),
                NumCompOp::Gre => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gre"),
                NumCompOp::Les => self.builder.build_int_compare(IntPredicate::SLT, l, r, "les"),
                NumCompOp::GrEq => self.builder.build_int_compare(IntPredicate::SGE, l, r, "geq"),
                NumCompOp::LeEq => self.builder.build_int_compare(IntPredicate::SLE, l, r, "leq"),
                
            }
        }

        fn compile_bin(&self, op: BinOp, l: IntValue, r: IntValue) -> IntValue{
            match op{
                BinOp::Add => self.builder.build_int_add(l, r, "add"),
                BinOp::Sub => self.builder.build_int_sub(l, r, "sub"),
                BinOp::Mul => self.builder.build_int_mul(l, r, "mul"),
                BinOp::Div => self.builder.build_int_signed_div(l, r, "div"),
            }
        }

        fn compile_bool(&self, b: bool) -> IntValue{
            if b {
                self.context.bool_type().const_int(1, false)
            } else {
                self.context.bool_type().const_int(0, false)
            }
        }

        fn compile_num(&self, num: i32) -> IntValue{
            let mut tmp_val: i32 = num;
            let neg = if tmp_val < 0 {tmp_val *= -1; true} else {false};

            let return_value = self.context.i32_type().const_int(tmp_val as u64, true);
            
            // if the value is supposed to be negative multiply by the IntValue 11111....
            // thus make it negative
            if neg{
                return_value.const_mul(self.context.i32_type().const_all_ones())
            }else{
                return_value
            }
        }
    }


    /// Convenience type alias for the `sum` function.
    ///
    /// Calling this is innately `unsafe` because there's no guarantee it doesn't
    /// do `unsafe` operations internally.
    type ExprFunc = unsafe extern "C" fn() -> i32;

    pub fn generate_llvm_code(program: Box<ExprTree>) -> Result<(), Box<Error>> {

        let context = Context::create();
        let module = context.create_module("program");
        let builder = context.create_builder();
        let execution_engine; 
        match module.create_jit_execution_engine(OptimizationLevel::None){
            Ok(e) => execution_engine = e,
            Err(err) => panic!("ERROR: can't init LLVM: {:?}", err),
        }


        let u32_type = context.i32_type();
        let fn_type = u32_type.fn_type(&[], false);
        let function = module.add_function("expr", fn_type, None);
        let basic_block = context.append_basic_block(&function, "entry");
        builder.position_at_end(&basic_block);

        let pass_manager = PassManager::create(&module);
        pass_manager.initialize();


        let mut llvm = LLVM{
            context: &context,
            module: &module,
            builder: &builder,
            fn_value_opt: Some(function),
            variables: HashMap::new(),
            
        };

        llvm.compile_block(program);

        let compiled_program: JitFunction<ExprFunc> =
            unsafe {execution_engine.get_function("expr").ok().unwrap()};

        pass_manager.run_on(&function);

        module.print_to_stderr();
        unsafe {
            println!("Whuut: {} ", compiled_program.call());
        }


        Ok(())
    }

}