
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

    let input = "let a: bool = true && 2>3 || 5 == 10;";

    //let expr = parser::ExprParser::new().parse(input).unwrap();

    let set = parser::SeparateLinesParser::new().parse(input).unwrap();

    println!("{:#?}", set);

  
}

#[test]
fn testExpr(){


    assert_eq!(parser::ExprParser::new().parse("1+2").unwrap(), Box::new(
                                                                    crate::ast::ExprTree::BinNode(
                                                                        Box::new(crate::ast::ExprTree::Number(1)), 
                                                                        crate::ast::BinOp::Add,
                                                                        Box::new(crate::ast::ExprTree::Number(2)))));
    
    assert_eq!(parser::ExprParser::new().parse("1 * 2 - 2").unwrap(), Box::new(
                                                                        crate::ast::ExprTree::BinNode(
                                                                            Box::new(crate::ast::ExprTree::BinNode(
                                                                                Box::new(crate::ast::ExprTree::Number(1)),
                                                                                crate::ast::BinOp::Mul,
                                                                                Box::new(crate::ast::ExprTree::Number(2)),
                                                                            )), 
                                                                            crate::ast::BinOp::Sub,
                                                                            Box::new(crate::ast::ExprTree::Number(2)))));
}