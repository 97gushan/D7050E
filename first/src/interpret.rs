

pub mod interpreter{

    use lazy_static;
    
    use crate::ast::*;

    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static!{
        static ref MEMORY: Mutex<HashMap<&'static str, IntRep>> = {
            let m = HashMap::new();
            
            Mutex::new(m)
        };
    }


    #[derive(Debug, PartialEq)]
    enum IntRep{
        Number(i32),
        Var(String),
        Bool(bool),
        NewLine,
    }


    pub fn run(mut ast: Vec<Box<ExprTree>>){
        let len = ast.len();

        


        //println!("{:#?}", map.get("a"));

        for _i in 0..len{
            match ast.pop(){
                Some(e) => {
                    match_node(e);
                    //println!("{:#?}", match_node(e, &MEMORY));
                },
                None => panic!("Fuuuck"),

            }

        }
    }   


    fn match_node(ast: Box<ExprTree>) -> IntRep {
        match *ast{
            ExprTree::Number(num) =>  IntRep::Number(num),
            ExprTree::Var(name) =>  read_from_var(&name),
            ExprTree::Bool(b) => IntRep::Bool(b),
            ExprTree::BinNode(l,op,r) =>  eval_bin_op(op, match_node(l), match_node(r)),
            ExprTree::NumCompNode(l,op,r) => eval_num_comp_op(op, match_node(l), match_node(r)),
            ExprTree::LogNode(l, op, r) => eval_bool_comp_op(op, match_node(l), match_node(r)),
            ExprTree::SeqNode(l, r) => {match_node(l); match_node(r); IntRep::NewLine},
            ExprTree::Print(s) => {println!("{:#?}", match_node(s)); IntRep::NewLine},
            ExprTree::AssignNode(n, _t, val) => {
                match *n{
                    ExprTree::Var(name) => assign_var(IntRep::Var(name), match_node(val)),
                    _ => panic!("ERROR: Can't get variable name to assign")
                }
            }
            ExprTree::SetVarNode(n, val) => assign_existing_var(n, val),
                
            _ => {
                panic!("ERROR: Cant match node")
            }
        }
    }  

    fn assign_existing_var(n: Box<ExprTree>, val: Box<ExprTree>) -> IntRep{
        match *n{
            ExprTree::Var(name) => {

                let value = match_node(val);
                let saved_value = read_from_var(&name);

                // check if types match
                if std::mem::discriminant(&value) == std::mem::discriminant(&saved_value){
                    assign_var(IntRep::Var(name), value)
                }else{
                    panic!("ERROR: can't assign to var, different types");
                }
            },
            _ => panic!("ERROR: Can't get variable name to assign")
        }
    }

    fn read_from_var(name: &str) -> IntRep{
        let map = MEMORY.lock().unwrap();

        match map.get(&*name){
            Some(var) => {
                match var{
                    IntRep::Number(num) => IntRep::Number(*num), 
                    IntRep::Bool(b) => IntRep::Bool(*b),
                    _ => panic!("ERROR: Var is not i32 or bool")
                }
            },
            None => {panic!("ERROR: Can't read var");}
        }
    }

    fn assign_var(name: IntRep, val: IntRep) -> IntRep{
        
        match name{
            IntRep::Var(n) => {
                let mut map = MEMORY.lock().unwrap();
                
                map.insert(Box::leak(n.into_boxed_str()), val);                
            },
            _ => panic!("ERROR: Can't assign to var")
        }

        IntRep::NewLine
    }

    fn eval_bool_comp_op(op: LogOp, l: IntRep, r: IntRep) -> IntRep{
        let left: bool;
        let right: bool;

        match l {
            IntRep::Bool(b) => {left = b;},
            _ => {
                panic!("ERROR: Left expression not bool")
            }
        }
        
        match r {
            IntRep::Bool(b) => {right = b;},
            _ => {
                panic!("ERROR: Right expression not bool")
            }
        }

        match op{
            LogOp::And => IntRep::Bool(left && right),
            LogOp::Or => IntRep::Bool(left || right),
        }
    }

    fn eval_num_comp_op(op: NumCompOp, l: IntRep, r: IntRep) -> IntRep{

        let left: i32;
        let right: i32;

        match l {
            IntRep::Number(num) => {left = num;},
            _ => {
                panic!("ERROR: Left expression not a number")
            }
        }

        match r {
            IntRep::Number(num) => {right = num;},
            _ => {
                panic!("ERROR: Right expression not a number")
            }
        }

        match op{
            NumCompOp::Les => IntRep::Bool(left < right),
            NumCompOp::Gre => IntRep::Bool(left > right),
            NumCompOp::LeEq => IntRep::Bool(left <= right),
            NumCompOp::GrEq => IntRep::Bool(left >= right),
            NumCompOp::Eq => IntRep::Bool(left == right),
            NumCompOp::Neq => IntRep::Bool(left != right),
        }
    }

    fn eval_bin_op(op: BinOp, l: IntRep, r: IntRep) -> IntRep{

        let left: i32;
        let right: i32;


        match l {
            IntRep::Number(num) => {left = num;},
            _ => {panic!("ERROR: Left expression is not a number");}
        }

        match r {
            IntRep::Number(num) => {right = num;},
            _ => {panic!("ERROR: Right expression is not a number");}
        }

        match op{
            BinOp::Add => IntRep::Number(left + right),
            BinOp::Sub => IntRep::Number(left - right),
            BinOp::Mul => IntRep::Number(left * right),
            BinOp::Div => IntRep::Number(left / right),
        }
    }
}