pub mod memory_handler{

    use crate::ast::*;


    #[derive(Debug, PartialEq, Clone)]
    pub enum IntRep {
        Number(i32),
        Var(String),
        Const(Box<IntRep>),
        Bool(bool),
        Function(Vec<Box<ExprTree>>, Type, ExprTree),
        Undefined(Type),

        TypeError(String),

        NewLine,
    }

    use lazy_static;

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


    pub fn insert_function(name: String, function: IntRep){
        let mut map = FUNCTION_MAP.lock().unwrap();

        // insert the function into the hashmap
        map.insert(Box::leak(name.into_boxed_str()), function);
    }

     /**
     * Takes a function name and returns the ExprTree branch and params of that function
     */
    pub fn read_function(name: String) -> (ExprTree, Vec<Box<ExprTree>>){
        let map = FUNCTION_MAP.lock().unwrap();

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

    pub fn get_function_type(name: String) -> IntRep{
        let map = FUNCTION_MAP.lock().unwrap();

        let return_type;

        match map.get(&*name) {
            Some(fun) => match fun {
                IntRep::Function(_, r, _) => {
                    return_type = r    
                },
                _ => panic!("ERROR: Can't read function"),
            },
            None => {
                panic!("ERROR: No function with the name: {} ", name);
            }
        }

        match return_type{
            Type::I32 => IntRep::Number(0),
            Type::Bool => IntRep::Bool(false),
        }
    }


    pub fn push_on_return_stack(val: IntRep){
        let mut stack = STACK.lock().unwrap();
        stack.push(val);
    }

    pub fn pop_from_return_stack() -> IntRep{
        let mut stack = STACK.lock().unwrap();
        match stack.pop(){
            Some(i) => i,
            None => IntRep::NewLine
        }
    }

    /**
     * Push a memory scope
     */
    pub fn push_on_mem_stack(){
        let mut stack = SCOPE.lock().unwrap();

        stack.push(Mutex::new(HashMap::new()));
    }

    /**
     * Pop a memory scope
     */
    pub fn pop_from_mem_stack(){
        let mut stack = SCOPE.lock().unwrap();

        stack.pop();
    }

    pub fn read_from_var(name: &str) -> IntRep {
        let scope = SCOPE.lock().unwrap();

        match scope.last(){
            Some(m) => {
                let map = m.lock().unwrap();

                match map.get(&*name) {
                    Some(var) => match var {
                        IntRep::Number(num) => IntRep::Number(*num),
                        IntRep::Bool(b) => IntRep::Bool(*b),
                        IntRep::Undefined(t) => IntRep::Undefined(*t),
                        IntRep::Var(n) => IntRep::Var(n.to_string()),
                        IntRep::Const(val) => IntRep::Const((*val).clone()),
                        IntRep::TypeError(e) => IntRep::TypeError(e.to_string()),
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

    pub fn assign_var(name: IntRep, val: IntRep) -> IntRep {        
        println!("{:#?} {:#?}", name, val);
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

}
