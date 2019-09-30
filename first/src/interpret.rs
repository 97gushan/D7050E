pub mod interpreter {

    use lazy_static;
    use crate::ast::*;

    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static! {
        static ref MEMORY: Mutex<HashMap<&'static str, IntRep>> = {
            let m = HashMap::new();
            Mutex::new(m)
        };

        static ref SCOPE: Mutex<Vec<Mutex<HashMap<&'static str, IntRep>>>> = {
            let s = Vec::new();
            Mutex::new(s)
        };

        static ref FUNCTION_MAP: Mutex<HashMap<&'static str, IntRep>> = {
            let f = HashMap::new();
            Mutex::new(f)
        };

        static ref STACK: Mutex<Vec<IntRep>> = {
            let s = Vec::new();
            Mutex::new(s)
        };
    }

    #[derive(Debug, PartialEq, Clone)]
    enum IntRep {
        Number(i32),
        Var(String),
        Bool(bool),
        Function(Vec<Box<ExprTree>>, Type, ExprTree),
        Undefined(Type),

        NewLine,
    }

    pub fn run(mut ast: Vec<Box<ExprTree>>) {
        let len = ast.len();

        for _i in 0..len {
            match ast.pop() {
                Some(fun) => {
                    match_node(fun);
                }
                None => panic!("Fuuuck"),
            }
        }

        call_function("main".to_string(), Vec::new());
    }

    fn match_node(ast: Box<ExprTree>) -> IntRep {
        match *ast {
            ExprTree::Number(num) => IntRep::Number(num),
            ExprTree::Var(name) => read_from_var(&name),
            ExprTree::Bool(b) => IntRep::Bool(b),
            ExprTree::BinNode(l, op, r) => eval_bin_op(op, match_node(l), match_node(r)),
            ExprTree::NumCompNode(l, op, r) => eval_comp_op(op, match_node(l), match_node(r)),
            ExprTree::LogNode(l, op, r) => eval_bool_log_op(op, match_node(l), match_node(r)),
            ExprTree::SeqNode(l, r) => {
                match_node(l);
                match_node(r);
                IntRep::NewLine
            }
            ExprTree::Print(s) => {
                println!("{:#?}", match_node(s));
                IntRep::NewLine
            }
            ExprTree::AssignNode(n, _t, val) => match *n {
                ExprTree::Var(name) => assign_var(IntRep::Var(name), match_node(val)),
                _ => panic!("ERROR: Can't get variable name to assign"),
            },
            ExprTree::SetVarNode(n, val) => assign_existing_var(n, val),
            ExprTree::IfNode(c, b) => eval_if_statement(match_node(c), b),
            ExprTree::IfElseNode(c, bi, be) => eval_if_else_statement(match_node(c), bi, be),
            ExprTree::WhileNode(c, b) => eval_while(c, b),
            ExprTree::FnNode(n, p, r, b) => eval_function(n, p, r, *b),
            ExprTree::ParamNode(n, t) => match *n {
                ExprTree::Var(name) => assign_var(IntRep::Var(name), IntRep::Undefined(t)),
                _ => panic!("ERROR: Can't get variable name to assign"),
            },
            ExprTree::FunctionCall(n, p) => call_function(get_function_name(n), get_function_params(p)),
            ExprTree::Return(e) => return_function(match_node(e)),
            _ => panic!("ERROR: Cant match node"),
        }
    }

    /**
     * Takes a function and stores it in the hashmap
     */
    fn eval_function(fun_name: FnHead, fun_params: FnHead, fun_return : FnHead, b: ExprTree) -> IntRep{
        
        let name = get_function_name(fun_name);
        let params = get_function_params_expr(fun_params);
        let return_type = get_function_return_type(fun_return);

        let mut map = FUNCTION_MAP.lock().unwrap();

        // insert the function into the hashmap
        map.insert(Box::leak(name.into_boxed_str()), IntRep::Function(params, return_type, b));

        IntRep::NewLine        
    }

    /**
     * Takes a function name and returns the ExprTree branch and params of that function
     */
    fn read_function(name: String) -> (ExprTree, Vec<Box<ExprTree>>){
        let map = FUNCTION_MAP.lock().unwrap();

        println!("function call {}", name);

        match map.get(&*name) {
            Some(fun) => match fun {
                IntRep::Function(p, _r, b) => {
                    
                    // return the branch and the params
                    (b.clone(), p.clone())    
                },
                _ => panic!("ERROR: Can't read function"),
            },
            None => {
                panic!("ERROR: No function with the name: {} ", name);
            }
        }
    }

    fn push_on_return_stack(val: IntRep){
        let mut stack = STACK.lock().unwrap();
        stack.push(val);
    }

    fn pop_from_return_stack() -> IntRep{
        let mut stack = STACK.lock().unwrap();
        match stack.pop(){
            Some(i) => i,
            None => IntRep::NewLine
        }
    }

    /**
     * Push a memory scope
     */
    fn push_on_mem_stack(){
        let mut stack = SCOPE.lock().unwrap();

        stack.push(Mutex::new(HashMap::new()));
    }

    /**
     * Pop a memory scope
     */
    fn pop_from_mem_stack(){
        let mut stack = SCOPE.lock().unwrap();

        stack.pop();
    }

    /**
     * Takes a function name and a vector of args
     * and calls the named function and stores the args
     * as variables in the hashmap
     */
    fn call_function(name: String, args: Vec<IntRep>) -> IntRep {

        // create a new scope
        push_on_mem_stack();

        // get branch and params from the functions
        let (branch, params) = read_function(name);

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

            assign_var(name, args[i].clone());
        }

        match_node(Box::new(branch.clone()));

        pop_from_mem_stack();
        pop_from_return_stack()
    }

    fn return_function(return_val: IntRep) -> IntRep{
        push_on_return_stack(return_val);
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

    fn assign_existing_var(n: Box<ExprTree>, val: Box<ExprTree>) -> IntRep {
        match *n {
            ExprTree::Var(name) => {
                let value = match_node(val);
                let saved_value = read_from_var(&name);

                // check if types match
                if std::mem::discriminant(&value) == std::mem::discriminant(&saved_value) {
                    assign_var(IntRep::Var(name), value)
                } else {
                    panic!("ERROR: can't assign to var, different types");
                }
            }
            _ => panic!("ERROR: Can't get variable name to assign"),
        }
    }

    fn read_from_var(name: &str) -> IntRep {
        let scope = SCOPE.lock().unwrap();

        match scope.last(){
            Some(m) => {
                let map = m.lock().unwrap();
                match map.get(&*name) {
                    Some(var) => match var {
                        IntRep::Number(num) => IntRep::Number(*num),
                        IntRep::Bool(b) => IntRep::Bool(*b),
                        IntRep::Undefined(t) => IntRep::Undefined(*t),
                        _ => panic!("ERROR: Var is not i32 or bool"),
                    },
                    None => {
                        panic!("ERROR: Var not found in scope");
                    }
                }
            }, 
                
            None => panic!("ERROR: No scope found"),
        }
    }

    fn assign_var(name: IntRep, val: IntRep) -> IntRep {        
        match name {
            IntRep::Var(n) => {
                let scope = SCOPE.lock().unwrap();

                match scope.last(){
                    Some(m) => {
                        let mut map = m.lock().unwrap();
                        map.insert(Box::leak(n.into_boxed_str()), val);
                    },
                        
                    None => {panic!("ERROR: No scope found");},
                }
            }
            _ => panic!("ERROR: Can't assign to var"),
        }

        IntRep::NewLine
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