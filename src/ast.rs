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
pub enum ClassNameReference {
    Identifier(String)
}

#[derive(Debug)]
pub enum Expression {
    Variable(Box<Variable>),
    Expression(Box<Expression>),
    Increment(Box<Variable>, bool), // flag defines right / left hand.
    Decrement(Box<Variable>, bool), // flag defines right / left hand.
    UnaryOp { op: String, expr: Box<Expression> },
    BinaryOp { op: String, expr_left: Box<Expression>, expr_right: Box<Expression> },
    AssignOp { op: String, var: Box<Variable>, expr_right: Box<Expression> },
    CloneOp { expr: Box<Expression> },
    ExpressionOp {op: String, expr_left: Box<Expression>, expr_right: Box<Expression> },
    Instanceof {class_name_reference: Box<ClassNameReference>, expr: Box<Expression> },
    CastInt(Box<Expression>),
    CastDouble(Box<Expression>),
    CastString(Box<Expression>),
    CastArray(Box<Expression>),
    CastObject(Box<Expression>),
    CastBool(Box<Expression>),
    CastUnset(Box<Expression>),
    Parentness(Box<Expression>),
    New(Box<ClassNameReference>),
    Conditional { expr: Box<Expression>, expr_if: Box<Option<Box<Expression>>>, expr_else: Box<Expression> }
}
