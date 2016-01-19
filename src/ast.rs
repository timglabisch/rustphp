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
    Identifier(String),
    Expression(Box<Expression>)
}

#[derive(Debug)]
pub enum Expression {
    Variable(Box<Variable>),
    Expression(Box<Expression>),
    Increment(Box<Variable>, bool), // flag defines right / left hand.
    Decrement(Box<Variable>, bool), // flag defines right / left hand.
    AssignOp { op: String, var: Box<Variable>, expr_right: Box<Expression> }
}
