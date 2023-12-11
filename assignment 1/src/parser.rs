use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression {
    /// '\' <Variable> <Expression>
    Lambda(String, Box<Expression>),
    /// <Expression> <Expression>
    Application(Box<Expression>, Box<Expression>),
    /// <Variable>
    Variable(String),
    /// '(' <Expression> ')'
    Paren(Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ParseError {
    EmptyExpression,
    InvalidExpression,
}

pub(super) type Result<T> = std::result::Result<T, ParseError>;

pub(super) fn parse(tokens: Vec<Token>) -> Result<Expression> {
    if tokens.is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut expression: Vec<Expression> = Vec::new();

    todo!()
}
