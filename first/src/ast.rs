
#[derive(Debug, PartialEq)]
pub enum ExprTree{
    Number(i32),
    Bool(Bool),
    Var(String),
    BinNode(Box<ExprTree>, BinOp, Box<ExprTree>),
    LogNode(Box<ExprTree>, LogOp, Box<ExprTree>),
    AssignNode(Box<ExprTree>, Type, Box<ExprTree>),
    SeqNode(Box<ExprTree>, Box<ExprTree>),
    
    IfNode(Box<ExprTree>, Box<ExprTree>, Box<ExprTree>),

}

#[derive(Debug, PartialEq)]
pub enum Type{
    I32,
    Bool
}

#[derive(Debug, PartialEq)]
pub enum BinOp{
    Add,
    Sub,
    Div,
    Mul 
}


#[derive(Debug, PartialEq)]
pub enum LogOp{
    And,
    Or,
    Not,
    Les,
    Gre,
    LeEq,
    GrEq,
    Eq,
    NoEq,
}

#[derive(Debug, PartialEq)]
pub enum Bool{
    True,
    False,
}
