#[derive(Debug)]
pub struct If {
    condition: Box<Expression>,
    if_block : Box<Block>
}

impl If {
    pub fn new(expression: Box<Expression>, if_block: Box<Block>) -> If {
        If { condition: expression, if_block: if_block }
    }
}


#[derive(Debug)]
pub struct Block;

#[derive(Debug)]
pub enum Variable {
    Variable(Box<Variable>),
    Identifier(Box<String>),
    Expression(Box<Expression>)
}

#[derive(Debug)]
pub enum Expression {
    Variable(Box<Variable>),
    Expression(Box<Expression>),
    AssignOp { op: Box<String>, expr_left: Box<Expression>, expr_right: Box<Expression> }
}
