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
pub struct Expression;

#[derive(Debug)]
pub struct Block;
