pub mod llvm_generator{


    use inkwell::OptimizationLevel;
    use inkwell::builder::Builder;
    use inkwell::context::Context;
    use inkwell::execution_engine::{ExecutionEngine, JitFunction};
    use inkwell::module::Module;
    use inkwell::targets::{InitializationConfig, Target};
    use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
    use std::error::Error;
    use std::convert::TryInto;


    use crate::ast::{ExprTree, BinOp, LogOp, NumCompOp};


    pub struct LLVM{
        context: Context,
        module: Module,
        builder: Builder,
        execution_engine: ExecutionEngine,
    }

    impl LLVM{

        fn match_node(&self, expr: &ExprTree) -> IntValue{
            match expr{
                ExprTree::Number(num) => self.compile_num(*num),
                ExprTree::BinNode(l, op, r) => self.compile_bin(*op, self.match_node(l), self.match_node(r)),
                _ => panic!("")
            }
        }

        fn compile_bin(&self, op: BinOp, l: IntValue, r: IntValue) -> IntValue{
            match op{
                BinOp::Add => self.builder.build_int_add(l, r, "sum"),
                BinOp::Sub => self.builder.build_int_sub(l, r, "diff"),
                BinOp::Mul => self.builder.build_int_mul(l, r, "prod"),
                BinOp::Div => self.builder.build_int_signed_div(l, r, "quota"),
            }
        }

        fn compile_num(&self, num: i32) -> IntValue{
            let mut tmp_val: i32 = num;
            let neg = if tmp_val < 0{tmp_val *= -1; true} else {false};

            let return_value = self.context.i64_type().const_int(tmp_val as u64, true);
            
            // if the value is supposed to be negative multiply by the IntValue 11111....
            // thus make it negative
            if neg{
                return_value.const_mul(self.context.i64_type().const_all_ones())
            }else{
                return_value
            }
        }


        fn jit_compile_program(&self, expr: &ExprTree) -> Option<JitFunction<ExprFunc>>  {
            let i64_type = self.context.i64_type();
            let fn_type = i64_type.fn_type(&[i64_type.into()], false);

            let function = self.module.add_function("sum", fn_type, None);
            let basic_block = self.context.append_basic_block(&function, "entry");

            self.builder.position_at_end(&basic_block);

            let res = self.match_node(expr);

            self.builder.build_return(Some(&res));

            unsafe { self.execution_engine.get_function("sum").ok() }
        }
    }

    pub fn init_llvm() -> (Context, Module, Builder, ExecutionEngine){
        let context = Context::create();
        let module = context.create_module("program");
        let builder = context.create_builder();

        let execution_engine; 
        match module.create_jit_execution_engine(OptimizationLevel::None){
            Ok(e) => execution_engine = e,
            Err(err) => panic!("ERROR: can't init LLVM: {:?}", err),
        }

        (context, module, builder, execution_engine)
    }


    /// Convenience type alias for the `sum` function.
    ///
    /// Calling this is innately `unsafe` because there's no guarantee it doesn't
    /// do `unsafe` operations internally.
    type ExprFunc = unsafe extern "C" fn() -> i32;

    pub fn generate_llvm_code(ast: ExprTree) -> Result<(), Box<Error>> {
        
        let llvm_params = init_llvm();

        let llvm = LLVM{
            context: llvm_params.0,
            module: llvm_params.1,
            builder: llvm_params.2,
            execution_engine: llvm_params.3
            };
        

        let program = llvm.jit_compile_program(&ast)
            .ok_or("Unable to JIT compile `sum`")?;


        unsafe {
            println!("{} ", program.call());
        }

        Ok(())
    }

}