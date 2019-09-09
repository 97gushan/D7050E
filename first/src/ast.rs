
#[derive(Debug)]
pub enum ExprTree{
    Number(i32),
    Node(Box<ExprTree>, Op, Box<ExprTree>),
}

#[derive(Debug)]
pub enum Op{
    Add,
    Sub,
    Div,
    Mul 
}
