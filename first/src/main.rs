extern crate nom;

use nom::{
    IResult,
    character::complete::{digit1},
    FindSubstring};


#[derive(Debug)]
enum Tree {
    Cons(char, Box<Tree>, Box<Tree>),
    Leef(i32),
    Nil,
}

use crate::Tree::{Cons, Nil, Leef};


/// Creates a new node in the tree
/// 
/// # Arguments
/// 
/// * `input` - A substring that will be split into a node and subtrees
/// * `i` - Index of the operator that will act as node
fn create_node(input: &str, i: usize) -> Tree{
    
    let operator: char = input[i..i+1].parse().unwrap();

    let left_tree: Tree = create_tree(&input[..i]);
    let right_tree: Tree = create_tree(&input[i+1..]);

    return Cons(operator, Box::new(left_tree), Box::new(right_tree));
}


fn create_leef(input: &str) -> Tree{

    let parsed: IResult<&str, &str> = digit1(input);

    match parsed{
        Ok(result) => Leef(result.1.parse().unwrap()),
        Err(_error) => Nil
    }
}


fn create_tree(input: &str) -> Tree{

    let node: Tree;

    // find operators with +- first so the come higher in the tree
    // this so when the tree is evaluated the lowest nodes are done first 
    // this means that if all the */ gets evaluated before +-
    if let Some(i) = input.find_substring("+"){        
        node = create_node(input, i);
    }
    else if let Some(i) = input.find_substring("-"){        
        node = create_node(input, i);
    }
    else if let Some(i) = input.find_substring("*"){
        node = create_node(input, i);
    }
    else if let Some(i) = input.find_substring("/"){        
        node = create_node(input, i);
    }
    else{
        node = create_leef(input);
    }

    return node;
}


fn main() {

    let input = "101*2*3+2/44+1*2+3-23*5/2";
    
    let tree = create_tree(input);

    println!("{:#?}", tree);
}