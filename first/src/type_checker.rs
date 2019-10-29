
pub mod checker {

    use crate::ast::*;

    use crate::memory::*;    

    use crate::memory::memory_handler::IntRep;


    pub fn run(mut ast: Vec<Box<ExprTree>>) -> bool{
        let len = ast.len();


        memory_handler::push_on_mem_stack();

        for _i in 0..len {
            match ast.pop() {
                Some(fun) => {
                    match_node(fun);
                },
                None => panic!("Fuuuck"),
            }
        }
        
        match call_function("main".to_string(), Vec::new()){
            IntRep::TypeError(e) => {println!("{}", e); false},
            _ => true
        }
    }

    fn match_node(ast: Box<ExprTree>) -> IntRep {

        match *ast{

            // types
            ExprTree::Var(n) => memory_handler::read_from_var(&n),
            ExprTree::Number(_) => IntRep::Number(0),
            ExprTree::Bool(_) => IntRep::Bool(false),
            ExprTree::Const(val) => IntRep::Const(Box::new(match_node(val))),

            
            // expressions of different kinds
            ExprTree::BinNode(l, _op, r) => eval_bin_node(match_node(l), match_node(r)),
            ExprTree::NumCompNode(l, _op, r) => eval_num_comp_node(match_node(l), match_node(r)),
            ExprTree::LogNode(l, _op, r) => eval_log_node(match_node(l), match_node(r)),
            
            // variables
            ExprTree::AssignNode(n, t, val) => eval_assign_node(n, t, val),
            ExprTree::SetVarNode(n, val) => assign_existing_var(n, val),
            
            // functions
            ExprTree::FnNode(n, p, r, b) => eval_function(n, p, r, *b),
            ExprTree::FunctionCall(n, p) => call_function(get_function_name(n), get_function_params(p)),
            ExprTree::Return(e) => return_function(match_node(e)),
            ExprTree::ParamNode(n, t) => match *n {
                ExprTree::Var(name) => memory_handler::assign_var(IntRep::Var(name), IntRep::Undefined(t)),
                _ => panic!("ERROR: Can't get variable name to assign"),
            }

            // conditions
            ExprTree::WhileNode(c, b) => eval_cond_branch(match_node(c), match_node(b)),
            ExprTree::IfNode(c, b) => eval_cond_branch(match_node(c), match_node(b)),
            ExprTree::IfElseNode(c, b1, b2) => eval_cond_mult_branch(match_node(c), match_node(b1), match_node(b2)),
            
            // good to have stuff
            ExprTree::SeqNode(l, r) => eval_seq_node(match_node(l), match_node(r)),
            ExprTree::Print(b) => match_node(b),
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
        let (branch, params) = memory_handler::read_function(name.clone());

        if params.len() != args.len(){
            panic!("ERROR: Wrong amount of arguments. Expected {} found {}", params.len(), args.len());
        }

        // fetch return type and push it to the stack for later comparision with return value
        let return_type = memory_handler::get_function_type(name.clone());
        memory_handler::push_on_return_stack(return_type);

        // var to hold a TypeError if there are any in the args 
        let mut arg_type_checker = IntRep::NewLine;

        // assign the params as variables with the values from the args
        for i in 0..params.len(){

            let name;
            let var_type;

            match &*(params[i]){
                ExprTree::ParamNode(n, t) => {
                    match &**n{
                        ExprTree::Var(na) => {
                            var_type = t;
                            name = IntRep::Var(na.to_string())
                        }
                        _ => panic!("ERROR: Value is not a variable"),
                    };
                },
                _ => panic!("ERROR: Value is not a parameter"),
            }

            // check so the types of args and params match
            match (&args[i], var_type){
                (IntRep::Number(_), Type::I32) => {memory_handler::assign_var(name, args[i].clone());},
                (IntRep::Bool(_), Type::Bool) =>  {memory_handler::assign_var(name, args[i].clone());},
                _ => arg_type_checker = IntRep::TypeError("ERROR: params and args type does not match".to_string())
            }
        }

        let function_type = match_node(Box::new(branch.clone()));

        memory_handler::pop_from_mem_stack();
        let return_type = memory_handler::pop_from_return_stack();

        // if there is a TypeError anywhere in the function, return a TypeError
        match (&function_type, &return_type, &arg_type_checker){
            (IntRep::TypeError(e), _, _) => IntRep::TypeError(e.to_string()),
            (_, IntRep::TypeError(e), _) => IntRep::TypeError(e.to_string()),
            (_, _, IntRep::TypeError(e)) => IntRep::TypeError(e.to_string()),
            _ => return_type,
        }
    }

    fn return_function(return_val: IntRep) -> IntRep{
        memory_handler::push_on_return_stack(return_val);
        IntRep::NewLine
    }

    // evaluate conditions with one branches like if and while
    fn eval_cond_branch(c: IntRep, b: IntRep) -> IntRep{
        match (c, b){
            (IntRep::TypeError(e), _) => IntRep::TypeError(e),
            (IntRep::Number(_), _) => IntRep::TypeError("ERROR: Wrong type in condition".to_string()),
            (_, IntRep::TypeError(e)) => IntRep::TypeError(e),
            _ => IntRep::NewLine,
        }
    }

    // evaluate conditions with multiple branches like if/else
    fn eval_cond_mult_branch(c: IntRep, b1: IntRep, b2: IntRep) -> IntRep{
        match (c, b1, b2){
            (IntRep::TypeError(e), _, _) => IntRep::TypeError(e),
            (IntRep::Number(_), _, _) => IntRep::TypeError("ERROR: Wrong type in condition".to_string()),
            (_, IntRep::TypeError(e), _) => IntRep::TypeError(e),
            (_, _,IntRep::TypeError(e)) => IntRep::TypeError(e),
            _ => IntRep::NewLine,
        }
    }

    fn eval_seq_node(l: IntRep, r: IntRep) -> IntRep{
        match (l, r){
            (IntRep::TypeError(e), _) => IntRep::TypeError(e),
            (_, IntRep::TypeError(e)) => IntRep::TypeError(e),
            _ => IntRep::NewLine,
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

    // fn eval_assign_node(name: Box<ExprTree>, t: Type, val: Box<ExprTree>) -> IntRep{

    //     let value = match_node(val);

    //     // check so var type and value are of same type
    //     check_var_types(name, t, value)  
    // }

    // fn check_var_types(name: Box<ExprTree>, t: Type, val: IntRep) -> IntRep{
    //     match (&val, t){
    //         (IntRep::Number(_), Type::I32) => assign_var(name, val),
    //         (IntRep::Bool(_), Type::Bool) => assign_var(name, val),
    //         (IntRep::Const(var), _) => assign_const(name, t, (**var).clone()),
    //         _ => IntRep::TypeError("ERROR: missmatched variable types".to_string()),

    //     }
    // }

    // fn assign_const(name: Box<ExprTree>,t: Type, val: IntRep) -> IntRep{
    //     match (&val, t){
    //         (IntRep::Number(_), Type::I32) => assign_var(name, IntRep::Const(Box::new(val))),
    //         (IntRep::Bool(_), Type::Bool) => assign_var(name, IntRep::Const(Box::new(val))),
    //         (IntRep::Const(var), _t) => assign_const(name, t, (**var).clone()),
    //         _ => IntRep::TypeError("ERROR: missmatched variable types".to_string()),

    //     }
    // }

    // fn assign_var(name: Box<ExprTree>, val: IntRep) -> IntRep{
    //     match *name {
    //         ExprTree::Var(name) => memory_handler::assign_var(IntRep::Var(name), val),
    //         _ => panic!("ERROR: Can't get variable name to assign"),
    //     }
    // }

    fn get_val_from_const(c: IntRep) -> IntRep{
         if let IntRep::Const(val) = c{
            *val
        }else{
            c
        }
    }

    fn eval_bin_node(l: IntRep, r: IntRep) -> IntRep{
 

        if std::mem::discriminant(&get_val_from_const(l)) == std::mem::discriminant(&get_val_from_const(r)) {
            IntRep::Number(0)
        }else{
            IntRep::TypeError("ERROR: missmatch types in binary operation".to_string())
        }
    }

    fn eval_num_comp_node(l: IntRep, r: IntRep) -> IntRep{
        if std::mem::discriminant(&get_val_from_const(l)) == std::mem::discriminant(&get_val_from_const(r)) {
            IntRep::Bool(true)
        }else{
            IntRep::TypeError("ERROR: missmatch types in compare operation".to_string())
        }
    }

    fn eval_log_node(l: IntRep, r: IntRep) -> IntRep{
        if std::mem::discriminant(&get_val_from_const(l)) == std::mem::discriminant(&get_val_from_const(r)) {
            IntRep::Bool(true)
        }else{
            IntRep::TypeError("ERROR: missmatch types in boolean operation".to_string())
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