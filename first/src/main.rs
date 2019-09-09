
pub mod ast;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser);



fn main() {

    //let input = "101*2*3+2/44+1*2+3* 10-23*5/2 + 2 + 1 *3 - 2 /5 *2 + 1";
    //let input = "1+3*(1 + 2) * 3 - -5 + 1 + __0this_is_camel_case";

    let input = "a = 2";

    //let expr = parser::ExprParser::new().parse(input).unwrap();

    let set = parser::LetParser::new().parse(input).unwrap();

    println!("{:#?}", set);

  
}