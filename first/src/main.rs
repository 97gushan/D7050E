mod ast;
mod parser_controller;
mod interpret;

#[macro_use] 
extern crate lalrpop_util;

#[macro_use]
extern crate lazy_static;

use crate::parser_controller::parser_mod;
use crate::interpret::interpreter;


fn main(){

    let ast = parser_mod::run_parser();
    println!("{:#?}", ast);
    interpreter::run(ast);
}