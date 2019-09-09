
#[derive(Debug)]
pub enum ExprTree{
    Number(i32),
    Var(String),
    BinNode(Box<ExprTree>, BinOp, Box<ExprTree>),
    EqNode(Box<ExprTree>, Box<ExprTree>),
}

#[derive(Debug)]
pub enum BinOp{
    Add,
    Sub,
    Div,
    Mul 
}


#[derive(Debug)]
pub enum LogOp{
    And,
    Or,
    Not,
    Eq
}
