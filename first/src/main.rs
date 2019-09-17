mod ast;
mod parser_controller;

#[macro_use] extern crate lalrpop_util;


use crate::parser_controller::parser_mod;



fn main(){

    parser_mod::run_parser();
    

}