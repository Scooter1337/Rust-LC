// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)
use crate::dbg;

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
    UnexpectedRParen,
    UnclosedLParen,
    NoAbstractionBody,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
            ParseError::UnexpectedRParen => write!(f, "Unexpected right parenthesis"),
            ParseError::UnclosedLParen => write!(f, "Unclosed left parenthesis"),
            ParseError::NoAbstractionBody => write!(f, "Missing abstraction body"),
        }
    }
}

pub(super) type ParseResult<T> = std::result::Result<T, ParseError>;

fn _parse(tokens: &[Token]) -> ParseResult<Expression> {
    let mut idx = 0;
    let mut result = Vec::new();

    while idx < tokens.len() {
        match &tokens[idx] {
            Token::Lambda(name) => {
                if idx + 1 >= tokens.len() {
                    return Err(ParseError::NoAbstractionBody);
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
                    dbg!(&tokens[end_idx]);
                    match tokens[end_idx] {
                        Token::LParen => paren_count += 1,
                        Token::RParen => {
                            if paren_count == 1 {
                                result.push(_parse(&tokens[idx + 1..end_idx])?);
                                dbg!(&result.last());
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
                    return Err(ParseError::UnclosedLParen);
                }
                idx = end_idx;
            }
            Token::RParen => {
                return Err(ParseError::UnexpectedRParen);
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
        // Reduce the result vector to a single expression
        match result.into_iter().reduce(|left_expr, right_expr| {
            Expression::Application(Box::new(left_expr), Box::new(right_expr))
        }) {
            Some(expr) => Ok(expr),
            None => Err(ParseError::InvalidExpression),
        }
    }
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
            Expression::Abstraction(name, expr) => write!(fmt, "λ{name}.{expr}"),
            /*
            If Variable -> print {name}
            Variable("a")

            Becomes

            a
             */
            Expression::Variable(name) => write!(fmt, "{name}"),
            /*
            If Application
                If lexpr = abs -> "({left_expr})"
                Else -> "{left_expr}"
                + " "
                + If rexpr = app | abs -> "({right_expr})"
                  Else -> "{right_expr}"
             */
            Expression::Application(left_expr, right_expr) => {
                // Left
                if let Expression::Abstraction(_l, _r) = left_expr.as_ref() {
                    write!(fmt, "({left_expr})")
                } else {
                    write!(fmt, "{left_expr}")
                }?;
                // Seperator
                write!(fmt, " ")?;
                // Right
                match right_expr.as_ref() {
                    Expression::Application(_, _) | Expression::Abstraction(_, _) => {
                        write!(fmt, "({right_expr})")
                    }

                    r_expr => write!(fmt, "{r_expr}"),
                }
            }
        }
    }
}

/// Parse the tokens into an expression
/// If given tokens result in an invalid expression, prints an error and exits the program
///
/// # Arguments
/// * `tokens` - The tokens to parse
/// * `idx` - The index of the line the tokens are on (for error printing), optional
///
/// # Returns
/// The parsed expression
///
/// # Error
/// * `idx` given - "Invalid expression [{err_code}] caught during parsing on line {idx}!"
/// * `idx` not given - "Invalid expression [{err_code}] caught during parsing!"
pub(crate) fn parse(tokens: &[Token], idx: Option<usize>) -> Expression {
    let expression = _parse(tokens);
    dbg!(&expression);
    match expression {
        // If error in expression, print error and exit
        Err(err_code) => {
            match idx {
                Some(idx) => {
                    eprintln!(
                        "Invalid expression [{}] caught during parsing on line {}!",
                        err_code,
                        idx + 1
                    );
                }
                None => {
                    eprintln!("Invalid expression [{}] caught during parsing!", err_code);
                }
            }

            std::process::exit(1);
        }
        // Else: reparse the expression (according to the assignment)
        Ok(expression) => expression,
    }
}

pub(crate) fn bench_parse(tokens: &[Token]) -> ParseResult<Expression> {
    _parse(tokens)
}
