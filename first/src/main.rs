#![allow(unused_imports)]
#![allow(dead_code)]

mod ast;
mod parser_controller;
mod interpret;
mod type_checker;
mod memory;
mod llvm;

#[macro_use] 
extern crate lalrpop_util;

#[macro_use]
extern crate lazy_static;

use crate::parser_controller::parser_mod;
use crate::interpret::interpreter;
use crate::type_checker::checker;
use crate::llvm::llvm_generator;
use std::convert::TryInto;



fn main(){
    
    let ast = parser_mod::run_parser("src/input.rs");
    println!("{:#?}", ast);

    if checker::run(ast.clone()){
        println!("Typechecker passed, interpret program");
        match llvm_generator::generate_llvm_code(ast.clone()){
            Ok(_) => (),
            Err(error) => panic!("-- {}", error),
        }
        //println!("{:#?}", interpreter::run(ast.clone()));
    }else{
        panic!("ERROR: Typechecker failed");
    }
}