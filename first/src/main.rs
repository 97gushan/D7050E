mod parser;

pub use crate::parser::parse_expression;


fn main() {

    //let input = "101*2*3+2/44+1*2+3* 10-23*5/2 + 2 + 1 *3 - 2 /5 *2 + 1";
    let input = "1 + 2 * 3 - 5";


    let tree = parse_expression::create_tree(input);

    println!("{:#?}", tree);
}