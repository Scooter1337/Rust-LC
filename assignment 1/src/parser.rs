use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expression {
    /// <Expression> <Expression>
    Application(Box<Expression>, Box<Expression>),
    /// <Variable>
    Variable(String),
    /// \ <Variable> . <Expression>
    Abstraction(String, Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ParseError {
    EmptyExpression,
    InvalidExpression,
}

pub(super) type Result<T> = std::result::Result<T, ParseError>;

fn _parse(tokens: &[Token]) -> Result<Expression> {
    let mut idx = 0;
    let mut result = Vec::new();

    while idx < tokens.len() {
        dbg!(&tokens[idx]);
        match &tokens[idx] {
            Token::Lambda(name) => {
                if idx + 1 >= tokens.len() {
                    return Err(ParseError::InvalidExpression);
                }

                let body = _parse(&tokens[idx + 1..])?;
                result.push(Expression::Abstraction(name.clone(), Box::new(body)));
                idx = tokens.len();
            }
            Token::Variable(ref variable) => {
                result.push(Expression::Variable(variable.clone()));
            }
            Token::LParen => {
                let mut paren_count = 1;
                let mut end_idx = idx + 1;
                while end_idx < tokens.len() {
                    match tokens[end_idx] {
                        Token::LParen => paren_count += 1,
                        Token::RParen => {
                            if paren_count == 1 {
                                result.push(_parse(&tokens[idx + 1..end_idx])?);
                            }
                            paren_count -= 1
                        }
                        _ => {}
                    }
                    if paren_count == 0 {
                        break;
                    }
                    end_idx += 1;
                }
                if paren_count != 0 {
                    return Err(ParseError::InvalidExpression);
                }

                idx = end_idx + 1;
            }
            Token::RParen => {
                return Err(ParseError::InvalidExpression);
            }
        }
        idx += 1;
    }

    if result.is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    if result.len() == 1 {
        Ok(result.pop().unwrap())
    } else {
        match result
            .into_iter()
            .reduce(|acc, expr| Expression::Application(Box::new(acc), Box::new(expr)))
        {
            Some(expr) => Ok(expr),
            None => Err(ParseError::InvalidExpression),
        }
    }
}

pub(super) fn parse(tokens: &[Token]) -> Result<Expression> {
    Ok(_parse(tokens))?
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
