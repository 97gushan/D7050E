extern crate nom;

use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n, is_a},
  combinator::map_res,
  sequence::tuple,
  character::complete::{digit1, char},
  error::ErrorKind};


fn from_string(input: &str) -> Result<i32, std::num::ParseIntError> {
  i32::from_str_radix(input, 10)
}


#[derive(Debug)]
enum Tree {
    Cons(char, Box<Tree>, Box<Tree>),
    Leef(i32),
    Nil,
}

use crate::Tree::{Cons, Nil, Leef};


fn create_tree(input: &str) -> Tree{

    let test = parse_digit(input);

    let left_tree: Tree;
    let right_tree: Tree;
    let operator: &str;
    let root;

    match test{
        Ok(i) =>{
            let b = i;
            
            let input = b.0;
            left_tree = Leef(from_string(b.1).unwrap());

            if input.len() > 0 {
                let a = parse_operator(input).unwrap();

                right_tree = create_tree(&a.0);
                operator = a.1;

                root = Cons(operator.parse().unwrap(), Box::new(left_tree), Box::new(right_tree));

            }else{
                root = left_tree;
            }
        },
        Err(error) =>{
            println!("{:?}", error);
            left_tree = Leef(101);
            root = left_tree;
        }
    }    

    return root;
}

fn parse_digit(input: &str) -> IResult<&str, &str>{
    digit1(input)
}

fn parse_operator(input: &str) -> IResult<&str, &str>{
    let (a, b) = tag("+")(input)?;

    Ok((a, b))
}


fn main() {

    let input = "101+1+2+3";

    
    let tree = create_tree(input);

    println!("{:#?}", tree);
}