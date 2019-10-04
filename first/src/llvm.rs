pub mod llvm_generator{


    use inkwell::OptimizationLevel;
    use inkwell::builder::Builder;
    use inkwell::context::Context;
    use inkwell::execution_engine::{ExecutionEngine, JitFunction};
    use inkwell::module::Module;
    use inkwell::targets::{InitializationConfig, Target};
    use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
    use std::error::Error;

    use crate::ast::ExprTree;

    /// Convenience type alias for the `sum` function.
    ///
    /// Calling this is innately `unsafe` because there's no guarantee it doesn't
    /// do `unsafe` operations internally.
    type ExprFunc = unsafe extern "C" fn() -> i32;

    pub fn generate_llvm_code(ast: ExprTree) -> Result<(), Box<Error>> {
        let context = Context::create();
        let module = context.create_module("sum");
        let builder = context.create_builder();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

        let program = jit_compile_program(&ast, &context, &module, &builder, &execution_engine)
            .ok_or("Unable to JIT compile `sum`")?;


        unsafe {
            println!("{} ", program.call());
        }

        Ok(())
    }

    fn compile_bin(expr: &ExprTree,
        context: &Context,
        module: &Module,
        builder: &Builder,
        ) {

            match expr{
                ExprTree::Number(num) => context.i64_type(*num, true),

            }
            
    }

    fn jit_compile_program(
        expr: &ExprTree,
        context: &Context,
        module: &Module,
        builder: &Builder,
        execution_engine: &ExecutionEngine,
    ) -> Option<JitFunction<ExprFunc>>  {
        let i64_type = context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);

        let function = module.add_function("sum", fn_type, None);
        let basic_block = context.append_basic_block(&function, "entry");

        builder.position_at_end(&basic_block);

        let res = compile_bin(expr, context, module, builder);

        builder.build_return(Some(&res));

        unsafe { execution_engine.get_function("sum").ok() }
    }
}