extern crate nom;

pub mod parse_expression{

    use nom::{
        IResult,
        character::complete::{digit1, space0},
        FindSubstring};


    #[derive(Debug)]
    pub enum Tree {
        Cons(Op, Box<Tree>, Box<Tree>),
        Leaf(i32),
    }

    #[derive(Debug)]
    pub enum Op{
        Add,
        Sub,
        Mult,
        Div,
    }


    /// Creates a new node in the tree
    /// 
    /// # Arguments
    /// 
    /// * `input` - A substring that will be split into a node and subtrees
    /// * `i` - Index of the operator that will act as node
    fn create_node(input: &str, i: usize) -> Tree{
        
        let operator_char: &str = &input[i..i+1];

        let operator: Op;
    
        if operator_char == "+"{
            operator = Op::Add;
        }else if operator_char == "-"{
            operator = Op::Sub;
        }else if operator_char == "*"{
            operator = Op::Mult;
        }else{
            operator = Op::Div;
        }

        let left_tree: Tree = create_tree(&input[..i]);
        let right_tree: Tree = create_tree(&input[i+1..]);

        return Tree::Cons(operator, Box::new(left_tree), Box::new(right_tree));
    }


    fn create_leaf(input: &str) -> Tree{

        let removed_whitespace: IResult<&str, &str> = space0(input);

        let trimmed_input;

        match removed_whitespace{
            Ok(res) => trimmed_input = res.0,
            Err(_error) => panic!("Panic when removing whitespace")
        }
        
        let parsed: IResult<&str, &str> = digit1(trimmed_input);

        match parsed{
            Ok(result) => Tree::Leaf(result.1.parse().unwrap()),
            Err(_error) => panic!("This is not a number!!!!")
        }
    }


    pub fn create_tree(input: &str) -> Tree{

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
            node = create_leaf(input);
        }

        return node;
    }
}