
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
                    println!("{:#?}", match_node(fun));
                },
                None => panic!("Fuuuck"),
            }
        }

        false
    }

    fn match_node(ast: Box<ExprTree>) -> IntRep {

        match *ast{
            ExprTree::Var(n) => memory_handler::read_from_var(&n),
            ExprTree::Number(_) => IntRep::Number(0),
            ExprTree::Bool(_) => IntRep::Bool(false),
            ExprTree::BinNode(l, _op, r) => eval_bin_node(match_node(l), match_node(r)),
            ExprTree::NumCompNode(l, _op, r) => eval_num_comp_node(match_node(l), match_node(r)),
            ExprTree::LogNode(l, _op, r) => eval_log_node(match_node(l), match_node(r)),
            ExprTree::AssignNode(n, _t, val) => match *n {
                ExprTree::Var(name) => memory_handler::assign_var(IntRep::Var(name), match_node(val)),
                _ => panic!("ERROR: Can't get variable name to assign"),
            },
            ExprTree::SetVarNode(n, val) => eval_set_var_node(n, val),
            ExprTree::SeqNode(l, r) => eval_seq_node(match_node(l), match_node(r)),
            

            ExprTree::ParamNode(n, t) => match *n {
                ExprTree::Var(name) => memory_handler::assign_var(IntRep::Var(name), IntRep::Undefined(t)),
                _ => panic!("ERROR: Can't get variable name to assign"),
            },
            _ => {println!("ERROR: Can't typecheck node"); IntRep::Undefined(Type::Bool)},
        }
    }

    fn eval_seq_node(l: IntRep, r: IntRep) -> IntRep{
        match (l, r){
            (IntRep::TypeError, _) => IntRep::TypeError,
            (_, IntRep::TypeError) => IntRep::TypeError,
            _ => IntRep::NewLine,
        }
    }

    fn eval_set_var_node(n: Box<ExprTree>, val: Box<ExprTree>) -> IntRep {
        match *n {
            ExprTree::Var(name) => {
                let value = match_node(val);
                let saved_value = memory_handler::read_from_var(&name);

                // check if types match
                if std::mem::discriminant(&value) == std::mem::discriminant(&saved_value) {
                    memory_handler::assign_var(IntRep::Var(name), value)
                } else {
                    IntRep::TypeError
                }
            }
            _ => panic!("ERROR: Can't get variable name to assign"),
        }
    }

    fn eval_bin_node(l: IntRep, r: IntRep) -> IntRep{
        if std::mem::discriminant(&l) == std::mem::discriminant(&r) {
            IntRep::Number(0)
        }else{
            IntRep::TypeError
        }
    }

    fn eval_num_comp_node(l: IntRep, r: IntRep) -> IntRep{
        if std::mem::discriminant(&l) == std::mem::discriminant(&r) {
            IntRep::Bool(true)
        }else{
            IntRep::TypeError
        }
    }

    fn eval_log_node(l: IntRep, r: IntRep) -> IntRep{
        if std::mem::discriminant(&l) == std::mem::discriminant(&r) {
            IntRep::Bool(true)
        }else{
            IntRep::TypeError
        }
    }
}	