
pub mod ast;



#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser);


fn main() {

    //let input = "101*2*3+2/44+1*2+3* 10-23*5/2 + 2 + 1 *3 - 2 /5 *2 + 1";
    //let input = "1+3*(1 + 2) * 3 - -5 + 1 + __0this_is_camel_case";
    //let input = "(1+2)";
    // let input = "let a = 2; 
    //              a = 3 + 3 - 2*5;
    //              let b = 10;";

    // let input = "if(true){
    //                 let a: i32 = 2;
    //             }else{
    //                 let k: i32 = 10;
    //             };
                
    //             let a: i32 = 10;
    //             let b: bool = 2==2 || 1 > 5;
    //             let c: i32 = 123;";

    let input = "while(true){ let a: i32 = 2;} let b: i32 = 123;";

    // let input = "if(true){ 
    //                 let a: i32 = 2;
                    
    //                 if(false){
    //                     let c: i32 = 132;
    //                 }

    //                 let b: i32 = 1234;
    //             }
                
    //             let a: i32 = 13;";       

    //let expr = parser::ExprParser::new().parse(input).unwrap();

    let set = parser::SeparateLinesParser::new().parse(input).unwrap();

    println!("{:#?}", set);  
}






#[cfg(test)]
mod test{

    use super::*;

    use crate::ast::*;

    #[test]
    fn test_expr(){


        assert_eq!(parser::ExprParser::new().parse("1+2").unwrap(), 
                Box::new(ExprTree::BinNode(
                            Box::new(ExprTree::Number(1)), 
                            BinOp::Add,
                            Box::new(ExprTree::Number(2)))));
        
        assert_eq!(parser::ExprParser::new().parse("1 * (2 - 2)").unwrap(), 
                Box::new( ExprTree::BinNode(
                            Box::new(ExprTree::Number(1)),                                                                                 
                            BinOp::Mul,
                            Box::new(ExprTree::BinNode(
                                Box::new(ExprTree::Number(2)),
                                BinOp::Sub,
                                Box::new(ExprTree::Number(2)),
                            )))));
    }
    
    #[test]
    fn test_bool(){
        assert_eq!(parser::BoolCompParser::new().parse("true").unwrap(), Box::new(ExprTree::Bool(BoolType::True)));

        assert_eq!(parser::BoolCompParser::new().parse("1 == 2 || 5 > 4 - 1").unwrap(), 
                Box::new(ExprTree::LogNode(
                        Box::new(ExprTree::LogNode(
                            Box::new(ExprTree::Number(1)),
                            LogOp::Eq,
                            Box::new(ExprTree::Number(2))
                        )),
                        LogOp::Or,
                        Box::new(ExprTree::LogNode(
                            Box::new(ExprTree::Number(5)),
                            LogOp::Gre,
                            Box::new(ExprTree::BinNode(
                                Box::new(ExprTree::Number(4)),
                                BinOp::Sub,
                                Box::new(ExprTree::Number(1))
                            ))
                        ))
                )));

        assert_eq!(parser::BoolCompParser::new().parse("true && false").unwrap(), 
                Box::new(ExprTree::LogNode(
                        Box::new(ExprTree::Bool(BoolType::True)),
                        LogOp::And,
                        Box::new(ExprTree::Bool(BoolType::False))
                )));
    }

    #[test]
    fn test_var(){
        assert_eq!(parser::SeparateLinesParser::new().parse("let a: i32 = 12;").unwrap(), 
                Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(12)))
            ));

        assert_eq!(parser::SeparateLinesParser::new().parse("let b: bool = true;").unwrap(), 
                Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("b".to_string())),
                        Type::Bool,
                        Box::new(ExprTree::Bool(BoolType::True)))
            ));


        assert_eq!(parser::SeparateLinesParser::new().parse("let b: bool = 1 >= 5;").unwrap(), 
                Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("b".to_string())),
                        Type::Bool,
                        Box::new(ExprTree::LogNode(
                            Box::new(ExprTree::Number(1)),
                            LogOp::GrEq,
                            Box::new(ExprTree::Number(5))
                        )))
            ));
    }

    #[test]
    fn test_seq(){
        assert_eq!(parser::SeparateLinesParser::new().parse("let a: i32 = 1; let b: bool = false;").unwrap(),
            Box::new(ExprTree::SeqNode(
                        Box::new(ExprTree::AssignNode(
                            Box::new(ExprTree::Var("a".to_string())),
                            Type::I32,
                            Box::new(ExprTree::Number(1))
                        )),
                        Box::new(ExprTree::AssignNode(
                            Box::new(ExprTree::Var("b".to_string())),
                            Type::Bool,
                            Box::new(ExprTree::Bool(BoolType::False))
                        ))

            ))
        );
    }

    #[test]
    fn test_if(){
        assert_eq!(parser::SeparateLinesParser::new().parse("if(true){ let a: i32 = 2;}").unwrap(), 
                Box::new(ExprTree::IfNode(
                    Box::new(ExprTree::Bool(BoolType::True)),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(2))
                    ))
                )));

        assert_eq!(parser::SeparateLinesParser::new().parse("if(true){ let a: i32 = 2;}else{let a: i32 = 2;}").unwrap(), 
                Box::new(ExprTree::IfElseNode(
                    Box::new(ExprTree::Bool(BoolType::True)),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(2))
                    )),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(2))
                    ))
                )));

        assert_eq!(parser::SeparateLinesParser::new().parse("if(true){ let a: i32 = 2;}else if(false){let a: i32 = 2;}else if(true){let a: i32 = 2;}").unwrap(), 
                Box::new(ExprTree::IfElseNode(
                    Box::new(ExprTree::Bool(BoolType::True)),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(2))
                    )),
                    Box::new(ExprTree::IfElseNode(
                        Box::new(ExprTree::Bool(BoolType::False)),
                        Box::new(ExprTree::AssignNode(
                            Box::new(ExprTree::Var("a".to_string())),
                            Type::I32,
                            Box::new(ExprTree::Number(2))
                        )),
                        Box::new(ExprTree::IfNode(
                            Box::new(ExprTree::Bool(BoolType::True)),
                            Box::new(ExprTree::AssignNode(
                                Box::new(ExprTree::Var("a".to_string())),
                                Type::I32,
                                Box::new(ExprTree::Number(2))
                            ))
                        ))
                    ))
                )));

        assert_eq!(parser::SeparateLinesParser::new().parse("if(true){ let a: i32 = 2;} let b: i32 = 123;").unwrap(), 
                Box::new(ExprTree::SeqNode(
                    Box::new(ExprTree::IfNode(
                        Box::new(ExprTree::Bool(BoolType::True)),
                        Box::new(ExprTree::AssignNode(
                            Box::new(ExprTree::Var("a".to_string())),
                            Type::I32,
                            Box::new(ExprTree::Number(2))
                        ))
                    )),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("b".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(123))
                    )))
                ));

        assert_eq!(parser::SeparateLinesParser::new().parse("if(true){ if(true){let a: i32 = 123;}}").unwrap(), 
                    Box::new(ExprTree::IfNode(
                        Box::new(ExprTree::Bool(BoolType::True)),
                        Box::new(ExprTree::IfNode(
                            Box::new(ExprTree::Bool(BoolType::True)),
                            Box::new(ExprTree::AssignNode(
                                Box::new(ExprTree::Var("a".to_string())),
                                Type::I32,
                                Box::new(ExprTree::Number(123))
                            )
                        ))
                    ))
                    
            ));
    }
}