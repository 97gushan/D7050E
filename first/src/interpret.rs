

pub mod interpreter{

    use lazy_static;
    
    use crate::ast::*;

    use std::collections::HashMap;
    use std::sync::Mutex;

    lazy_static!{
        static ref MEMORY: Mutex<HashMap<&'static str, i32>> = {
            let mut m = HashMap::new();
            
            Mutex::new(m)
        };
    }


    #[derive(Debug, PartialEq)]
    enum IntRep{
        Number(i32),
        Var(String),
        Bool(bool),
        Error(String),
        NewLine,
    }


    pub fn run(mut ast: Vec<Box<ExprTree>>){
        let len = ast.len();

        


        //println!("{:#?}", map.get("a"));

        for i in 0..len{
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
            ExprTree::Var(name) =>  IntRep::Var(name),
            ExprTree::BinNode(l,op,r) =>  eval_bin_op(op, match_node(l), match_node(r)),
            ExprTree::NumCompNode(l,op,r) => eval_num_comp_op(op, match_node(l), match_node(r)),
            ExprTree::SeqNode(l, r) => {match_node(l); match_node(r); IntRep::NewLine},
            ExprTree::Print(s) => {println!("{:#?}", match_node(s)); IntRep::NewLine},
            ExprTree::AssignNode(n, t, val) => assign_var(t, match_node(n), match_node(val)),
            _ => {
                IntRep::Error("Cant match node".to_string())
            }
        }
    }


    fn assign_var(var_type: Type, name: IntRep, val: IntRep) -> IntRep{
        let value;
        
        match val{
            IntRep::Number(n) => value = n,
            _ => panic!("Can't assign to bad value")
        }
        
        match name{
            IntRep::Var(n) => {
                let mut map = MEMORY.lock().unwrap();
                
                map.insert(Box::leak(n.into_boxed_str()), value);                
            },
            _ => panic!("Can't assign to var")
        }

        IntRep::NewLine
    }

    fn eval_bool_comp_op(op: NumCompOp, l: bool, r: bool) -> bool{
        match op{
            NumCompOp::Eq => (l == r),
            NumCompOp::Neq => (l != r),
            _ => false
        }
    }

    fn eval_num_comp_op(op: NumCompOp, l: IntRep, r: IntRep) -> IntRep{

        let left: i32;
        let right: i32;

        match l {
            IntRep::Number(num) => {left = num;},
            _ => {
                panic!("Error Not a number".to_string())
            }
        }

        match r {
            IntRep::Number(num) => {right = num;},
            _ => {
                panic!("Error Not a number".to_string())
            }
        }

        match op{
            NumCompOp::Les => IntRep::Bool(left < right),
            NumCompOp::Gre => IntRep::Bool(left > right),
            NumCompOp::LeEq => IntRep::Bool(left <= right),
            NumCompOp::GrEq => IntRep::Bool(left >= right),
            NumCompOp::Eq => IntRep::Bool(left == right),
            NumCompOp::Neq => IntRep::Bool(left != right),
            _ => IntRep::Error("Can't do comparison".to_string())
        }
    }

    fn eval_bin_op(op: BinOp, l: IntRep, r: IntRep) -> IntRep{

        let left: i32;
        let right: i32;


        match l {
            IntRep::Number(num) => {left = num;},
            IntRep::Var(name) => {
                let mut map = MEMORY.lock().unwrap();

                match map.get(&*name){
                    Some(num) => {left = *num},
                    None => {panic!("Can't read var");}
                }

            },
            _ => {panic!("Left: Not a number");}
        }

        match r {
            IntRep::Number(num) => {right = num;},
            IntRep::Var(name) => {
                
                let mut map = MEMORY.lock().unwrap();

                match map.get(&*name){
                    Some(num) => {right = *num},
                    None => {panic!("Can't read var");}
                }
            },
            _ => {panic!("Right: Not a number");}
        }

        match op{
            BinOp::Add => IntRep::Number(left + right),
            BinOp::Sub => IntRep::Number(left - right),
            BinOp::Mul => IntRep::Number(left * right),
            BinOp::Div => IntRep::Number(left / right),
        }
    }
}