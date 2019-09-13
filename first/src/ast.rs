
#[derive(Debug, PartialEq)]
pub enum ExprTree{
    Number(i32),
    Bool(BoolType),
    Var(String),

    BinNode(Box<ExprTree>, BinOp, Box<ExprTree>),
    LogNode(Box<ExprTree>, LogOp, Box<ExprTree>),
    
    AssignNode(Box<ExprTree>, Type, Box<ExprTree>),
    SeqNode(Box<ExprTree>, Box<ExprTree>),
    
    IfNode(Box<ExprTree>, Box<ExprTree>),
    IfElseNode(Box<ExprTree>, Box<ExprTree>, Box<ExprTree>),

    WhileNode(Box<ExprTree>, Box<ExprTree>),
    FnNode(FnHead, FnHead, FnHead, Box<ExprTree>),

    Pass,
}

#[derive(Debug, PartialEq)]
pub enum Type{
    I32,
    Bool
}

#[derive(Debug, PartialEq)]
pub enum FnHead{
    Name(String),
    Params(Vec<Box<ExprTree>>),
    Return(Type),
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
pub enum BoolType{
    True,
    False,
}
