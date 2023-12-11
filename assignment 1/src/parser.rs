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
    /// Vec<Expression>
    Body(Vec<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ParseError {
    EmptyExpression,
    InvalidExpression,
}

pub(super) type Result<T> = std::result::Result<T, ParseError>;

fn _parse(tokens: &Vec<Token>, i: &mut usize) -> Result<Expression> {
    if tokens.is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut expression: Vec<Expression> = Vec::new();

    while let Some(token) = tokens.get(*i) {
        match token {
            Token::Lambda(name) => {
                *i += 1;
                expression.push(Expression::Lambda(
                    name.to_owned(),
                    _parse(tokens, i)?.into(),
                ))
            }
            Token::Variable(name) => {
                expression.push(Expression::Variable(name.to_owned()));
            }
            Token::LParen => {
                *i += 1;
                expression.push(Expression::Paren(_parse(tokens, i)?.into()));
            }
            Token::RParen => {
                if expression.is_empty() {
                    return Err(ParseError::EmptyExpression);
                }
                return Ok(Expression::Body(expression));
            }
        }
        *i += 1;
    }
    if expression.is_empty() {
        return Err(ParseError::EmptyExpression);
    }
    Ok(Expression::Body(expression))
}

pub(super) fn parse(tokens: Vec<Token>) -> Result<Expression> {
    let parsed = _parse(&tokens, &mut 0);
    dbg!(&parsed);
    parsed
}

/*
Token::Lambda => {
            expression.push(Expression::Lambda(token, Box::new(body)));
        }
        Token::Variable(variable) => {
            expression.push(Expression::Variable(variable));
        }
        Token::OpenParen => {
            expression.push(Expression::Paren(Box::new(Expression::Variable(
                "(".to_string(),
            ))));
        }
        Token::CloseParen => {
            let mut body = Vec::new();
            while let Some(expr) = expression.pop() {
                match expr {
                    Expression::Variable(variable) if variable == "(" => {
                        break;
                    }
                    _ => {
                        body.push(expr);
                    }
                }
            }
            body.reverse();
            expression.push(Expression::Paren(Box::new(Expression::Application(
                Box::new(body[0].clone()),
                Box::new(body[1].clone()),
            ))));
        }

*/
