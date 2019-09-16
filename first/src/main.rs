pub mod ast;


#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser);

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let input: String;

    match read_src_file("src/input.txt"){
        Ok(content) => input = content,
        Err(_error) => input = String::from("")
    }

    println!("{}", &input);

    let set = parser::ProgramParser::new().parse(&input).unwrap();

    println!("{:#?}", set);  
}



fn read_src_file(src_path: &str) -> io::Result<(String)>{
    let mut file = File::open(src_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content);

    Ok(content)
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

        assert_eq!(parser::BoolCompParser::new().parse("a || b").unwrap(), 
                Box::new(ExprTree::LogNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        LogOp::Or,
                        Box::new(ExprTree::Var("b".to_string()))
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

    #[test]
    fn test_while(){

        assert_eq!(parser::SeparateLinesParser::new().parse("while(true){let a: i32 = 2;}").unwrap(),
            Box::new(ExprTree::WhileNode(
                Box::new(ExprTree::Bool(BoolType::True)),
                Box::new(ExprTree::AssignNode(
                    Box::new(ExprTree::Var("a".to_string())),
                    Type::I32,
                    Box::new(ExprTree::Number(2))
                ))
            )));

    }
    #[test]
    fn test_fun(){
         assert_eq!(parser::ProgramParser::new().parse("fn test() -> i32{let a: i32 = 123;}").unwrap(),
            vec!(Box::new(ExprTree::FnNode(
                FnHead::Name("test".to_string()),
                FnHead::Params(Vec::new()),
                FnHead::Return(Type::I32),
                Box::new(ExprTree::AssignNode(
                    Box::new(ExprTree::Var("a".to_string())),
                    Type::I32,
                    Box::new(ExprTree::Number(123))
                ))
            ))
        ));

        assert_eq!(parser::ProgramParser::new().parse("fn test(a: i32, b: bool) -> i32{let a: i32 = 123;}").unwrap(),
            vec!(Box::new(ExprTree::FnNode(
                FnHead::Name("test".to_string()),
                FnHead::Params(vec!(
                    Box::new(ExprTree::ParamNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32
                    )),
                    Box::new(ExprTree::ParamNode(
                        Box::new(ExprTree::Var("b".to_string())),
                        Type::Bool
                    )),

                )),
                FnHead::Return(Type::I32),
                Box::new(ExprTree::AssignNode(
                    Box::new(ExprTree::Var("a".to_string())),
                    Type::I32,
                    Box::new(ExprTree::Number(123))
                ))
            ))));

        assert_eq!(parser::ProgramParser::new().parse("fn foo() -> i32{if(true){let a: i32 = 123; } } fn bar() -> i32{let a: i32 = 123; }").unwrap(),
            vec![Box::new(ExprTree::FnNode(
                FnHead::Name("foo".to_string()),
                FnHead::Params(vec!()),
                FnHead::Return(Type::I32),
                Box::new(ExprTree::IfNode(
                    Box::new(ExprTree::Bool(BoolType::True)),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(123))
                        ))
                    )
                ))), 
                Box::new(ExprTree::FnNode(
                    FnHead::Name("bar".to_string()),
                    FnHead::Params(vec!()),
                    FnHead::Return(Type::I32),
                    Box::new(ExprTree::AssignNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Type::I32,
                        Box::new(ExprTree::Number(123))
                    ))
                )
                )
            ]);


        assert_eq!(parser::ProgramParser::new().parse("fn foo() -> i32{a = bar(32); bar(123, a);}").unwrap(),
            vec!(Box::new(ExprTree::FnNode(
                FnHead::Name("foo".to_string()),
                FnHead::Params(Vec::new()),
                FnHead::Return(Type::I32),
                Box::new(ExprTree::SeqNode(
                    Box::new(ExprTree::SetVarNode(
                        Box::new(ExprTree::Var("a".to_string())),
                        Box::new(ExprTree::FunctionCall(
                            FnHead::Name("bar".to_string()),
                            FnHead::Params(vec!(Box::new(ExprTree::Number(32))))
                        ))
                    )),
                    Box::new(ExprTree::FunctionCall(
                            FnHead::Name("bar".to_string()),
                            FnHead::Params(vec!(Box::new(ExprTree::Number(123)), Box::new(ExprTree::Var("a".to_string()))))
                        ))
                ))
            ))
        ));

        assert_eq!(parser::ProgramParser::new().parse("fn test() -> i32{return 5;}").unwrap(),
            vec!(Box::new(ExprTree::FnNode(
                FnHead::Name("test".to_string()),
                FnHead::Params(Vec::new()),
                FnHead::Return(Type::I32),
                Box::new(ExprTree::Return(Box::new(ExprTree::Number(5))))
            ))
        ));
    }

}