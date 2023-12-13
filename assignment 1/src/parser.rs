use std::fmt::{Display, Error, Formatter};

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

pub(super) type ParseResult<T> = std::result::Result<T, ParseError>;

fn _parse(tokens: &[Token]) -> ParseResult<Expression> {
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

pub(super) fn parse(tokens: &[Token]) -> ParseResult<Expression> {
    Ok(_parse(tokens))?
}

impl Display for Expression {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            /*
            If Lambda -> print λ{name}.{expr}
            Lambda(
                "x",
                Application ([
                Variable("a"),
                Variable("b")
            ])
            )

            Becomes

            λx.a b
            */
            Expression::Abstraction(name, expr) => write!(fmt, "λ{}.{}", name, expr),
            /*
            If Variable -> print {name}
            Variable("a")

            Becomes

            a
             */
            Expression::Variable(name) => write!(fmt, "{}", name),
            /*
            If Application
                If lexpr = abs -> "({left_expr})"
                Else -> "{left_expr}"
                + " "
                + If rexpr = app | abs -> "({right_expr})"
                  Else -> "{right_expr}"
             */
            Expression::Application(left_expr, right_expr) => {
                if let Expression::Abstraction(_l, _r) = left_expr.as_ref() {
                    write!(fmt, "({})", left_expr)?;
                } else {
                    write!(fmt, "{}", left_expr)?;
                };
                // Seperator
                write!(fmt, " ")?;
                match right_expr.as_ref() {
                    Expression::Application(_, _) | Expression::Abstraction(_, _) => {
                        write!(fmt, "({})", right_expr)
                    }

                    _ => write!(fmt, "{}", right_expr),
                }
            }
        }
    }
}
