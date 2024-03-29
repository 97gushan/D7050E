pub mod interpreter {

    use crate::ast::*;

    use crate::memory::*;    

    use crate::memory::memory_handler::IntRep;

    pub fn run(mut ast: Vec<Box<ExprTree>>) -> IntRep{
        let len = ast.len();

        for _i in 0..len {
            match ast.pop() {
                Some(fun) => {
                    match_node(fun);
                }
                None => panic!("Fuuuck"),
            }
        }

        call_function("main".to_string(), Vec::new())
    }

    fn match_node(ast: Box<ExprTree>) -> IntRep {
        match *ast {
            // types
            ExprTree::Number(num) => IntRep::Number(num),
            ExprTree::Var(name) => memory_handler::read_from_var(&name),
            ExprTree::Const(val) => IntRep::Const(Box::new(match_node(val))),
            ExprTree::Bool(b) => IntRep::Bool(b),
            
            // Expressions
            ExprTree::BinNode(l, op, r) => eval_bin_op(op, match_node(l), match_node(r)),
            ExprTree::NumCompNode(l, op, r) => eval_comp_op(op, match_node(l), match_node(r)),
            ExprTree::LogNode(l, op, r) => eval_bool_log_op(op, match_node(l), match_node(r)),
           
            // variables
            ExprTree::AssignNode(n, t, val) => eval_assign_node(n, t, val),
            ExprTree::SetVarNode(n, val) => assign_existing_var(n, val),
            
            // conditions
            ExprTree::IfNode(c, b) => eval_if_statement(match_node(c), b),
            ExprTree::IfElseNode(c, bi, be) => eval_if_else_statement(match_node(c), bi, be),
            ExprTree::WhileNode(c, b) => eval_while(c, b),
            
            // functions
            ExprTree::FnNode(n, p, r, b) => eval_function(n, p, r, *b),
            ExprTree::ParamNode(n, t) => match *n {
                ExprTree::Var(name) => memory_handler::assign_var(IntRep::Var(name), IntRep::Undefined(t)),
                _ => panic!("ERROR: Can't get variable name to assign"),
            },
            ExprTree::FunctionCall(n, p) => call_function(get_function_name(n), get_function_params(p)),
            ExprTree::Return(e) => return_function(match_node(e)),

            
            ExprTree::SeqNode(l, r) => {
                match_node(l);
                match_node(r);
                IntRep::NewLine
            }
            ExprTree::Print(s) => {
                println!("{:#?}", match_node(s));
                IntRep::NewLine
            }
            ExprTree::Pass => IntRep::NewLine,
        }
    }

    /**
     * Takes a function and stores it in the hashmap
     */
    fn eval_function(fun_name: FnHead, fun_params: FnHead, fun_return : FnHead, b: ExprTree) -> IntRep{
        
        let name = get_function_name(fun_name);
        let params = get_function_params_expr(fun_params);
        let return_type = get_function_return_type(fun_return);

        memory_handler::insert_function(name, IntRep::Function(params, return_type, b));
        
        IntRep::NewLine        
    }

   

    /**
     * Takes a function name and a vector of args
     * and calls the named function and stores the args
     * as variables in the hashmap
     */
    fn call_function(name: String, args: Vec<IntRep>) -> IntRep {

        // create a new scope
        memory_handler::push_on_mem_stack();

        // get branch and params from the functions
        let (branch, params) = memory_handler::read_function(name);

        if params.len() != args.len(){
            panic!("ERROR: Wrong amount of arguments. Expected {} found {}", params.len(), args.len());
        }

        // assign the params as variables with the values from the args
        for i in 0..params.len(){

            let name;

            match &*(params[i]){
                ExprTree::ParamNode(n, _t) => {
                    match &**n{
                        ExprTree::Var(na) => name = IntRep::Var(na.to_string()),
                        _ => panic!("ERROR: Value is not a variable"),
                    };
                },
                _ => panic!("ERROR: Value is not a parameter"),
            }

            memory_handler::assign_var(name, args[i].clone());
        }

        match_node(Box::new(branch.clone()));

        memory_handler::pop_from_mem_stack();
        memory_handler::pop_from_return_stack()
    }

    fn return_function(return_val: IntRep) -> IntRep{
        memory_handler::push_on_return_stack(return_val);
        IntRep::NewLine
    }

    fn get_bool_from_enum(comp: IntRep) -> bool {
        match comp {
            IntRep::Bool(b) => b,
            _ => panic!("ERROR: Value is not bool"),
        }
    }

    fn eval_while(comp: Box<ExprTree>, while_branch: Box<ExprTree>) -> IntRep {
        while get_bool_from_enum(match_node(comp.clone())) {
            match_node(while_branch.clone());
        }
        IntRep::NewLine
    }

    fn eval_if_statement(comp: IntRep, if_branch: Box<ExprTree>) -> IntRep {
        if let IntRep::Bool(c) = comp {
            if c {
                match_node(if_branch);
            }
        }

        IntRep::NewLine
    }

    fn eval_if_else_statement(comp: IntRep, 
                              if_branch: Box<ExprTree>, 
                              else_branch: Box<ExprTree>,) 
                              -> IntRep {
        if let IntRep::Bool(c) = comp {
            if c {
                match_node(if_branch);
            } else {
                match_node(else_branch);
            }
        }

        IntRep::NewLine
    }

     fn eval_assign_node(name: Box<ExprTree>, t: Type, val: Box<ExprTree>) -> IntRep{

        let value = match_node(val);

        // check so var type and value are of same type
        match (&value, t){
            (IntRep::Number(_), Type::I32) => assign_var(name, value),
            (IntRep::Bool(_), Type::Bool) => assign_var(name, value),
            (IntRep::Const(var), _) => {
                match ((**var).clone(), t){
                    (IntRep::Number(val), Type::I32) => assign_var(name, IntRep::Number(val)),
                    (IntRep::Bool(val), Type::Bool) => assign_var(name,  IntRep::Bool(val)),
                    _ => IntRep::TypeError("ERROR: Can't assign to var".to_string())
                }
            }
            _ => IntRep::TypeError("ERROR: Can't assign to var".to_string())
        }   
    }

    fn assign_var(name: Box<ExprTree>, val: IntRep) -> IntRep{
        match *name {
            ExprTree::Var(name) => memory_handler::assign_var(IntRep::Var(name), val),
            _ => panic!("ERROR: Can't get variable name to assign"),
        }
    }

    fn assign_existing_var(n: Box<ExprTree>, val: Box<ExprTree>) -> IntRep {
        match *n {
            ExprTree::Var(name) => {
                let mut value = match_node(val);
                let saved_value = memory_handler::read_from_var(&name);
                
                // if assign value is unmutable, get value from const node
                if let IntRep::Const(val) = value{
                    value = * val;
                }

                // check if types match
                if std::mem::discriminant(&value) == std::mem::discriminant(&saved_value) {
                    memory_handler::assign_var(IntRep::Var(name), value)
                } else {
                    panic!("ERROR: can't assign to var, different types");
                }
            },
            _ => panic!("ERROR: Can't get variable name to assign"),
        }
    }

    fn eval_bool_log_op(op: LogOp, l: IntRep, r: IntRep) -> IntRep {
        let left: bool;
        let right: bool;

        match (l, r) {
            (IntRep::Bool(l_bool), IntRep::Bool(r_bool)) => {
                left = l_bool;
                right = r_bool;
            }
            _ => panic!("ERROR: Missmatched types on bool operation"),
        }

        match op {
            LogOp::And => IntRep::Bool(left && right),
            LogOp::Or => IntRep::Bool(left || right),
        }
    }

    fn eval_comp_op(op: NumCompOp, l: IntRep, r: IntRep) -> IntRep {

        match(l, r){
            (IntRep::Number(l_num), IntRep::Number(r_num)) => eval_num_comp_op(op, l_num, r_num),
            (IntRep::Bool(l_bool), IntRep::Bool(r_bool)) => eval_bool_comp_op(op, l_bool, r_bool),
            _ => panic!("ERROR: Missmatched types on compare operation"),

        }
    }

    fn eval_bool_comp_op(op: NumCompOp, l: bool, r: bool) -> IntRep {
        match op {
            NumCompOp::Eq => IntRep::Bool(l == r),
            NumCompOp::Neq => IntRep::Bool(l != r),
            _ => panic!("ERROR: Can't do compare operator on bool"),
        }
    }

    fn eval_num_comp_op(op: NumCompOp, l: i32, r: i32) -> IntRep {
        match op {
            NumCompOp::Les => IntRep::Bool(l < r),
            NumCompOp::Gre => IntRep::Bool(l > r),
            NumCompOp::LeEq => IntRep::Bool(l <= r),
            NumCompOp::GrEq => IntRep::Bool(l >= r),
            NumCompOp::Eq => IntRep::Bool(l == r),
            NumCompOp::Neq => IntRep::Bool(l != r),
        }
    }

    fn eval_bin_op(op: BinOp, l: IntRep, r: IntRep) -> IntRep {
        let left: i32;
        let right: i32;

        match (l, r) {
            (IntRep::Number(left_num), IntRep::Number(right_num)) => {
                left = left_num;
                right = right_num;
            },
            _ => panic!("ERROR: Left expression is not a number"),
        }

        match op {
            BinOp::Add => IntRep::Number(left + right),
            BinOp::Sub => IntRep::Number(left - right),
            BinOp::Mul => IntRep::Number(left * right),
            BinOp::Div => IntRep::Number(left / right),
        }
    }

    fn get_function_name(fun_name: FnHead) -> String{
        match fun_name{
            FnHead::Name(n) => n,
            _ => panic!("ERROR: Can't read function name"), 
        }
    }

    fn get_function_params_expr(fun_params: FnHead) -> Vec<Box<ExprTree>>{
        match fun_params{
            FnHead::Params(p) => p,
            _ => panic!("ERROR: Can't read function params")
        }
    }

    fn get_function_params(fun_params: FnHead) -> Vec<IntRep>{
        let params = get_function_params_expr(fun_params);

        let mut params_int = Vec::new();

        for item in &params{
            params_int.push(match_node(item.clone()));
        }

        params_int

    }

    fn get_function_return_type(fun_return: FnHead) -> Type{
        match fun_return{
            FnHead::Return(r) => r,
            _ => panic!("ERROR: Can't read function return type")
        }
    }
}