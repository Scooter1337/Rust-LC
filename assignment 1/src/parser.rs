// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)
use crate::dbg;
use crate::tokenizer::Token;

use std::fmt::{Display, Formatter, Result};

// Boxes are heap allocated, so we can use them to store the expression tree
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
                // If lambda is the last token, return an error
                if idx + 1 >= tokens.len() {
                    return Err(ParseError::NoAbstractionBody);
                }

                // 28 DEC edit: precedence rules are other way around. \x a b = (λx.a) b and not λx.a b
                // Therefore, we find the end of the abstraction body first, and then recursively parse the body
                let mut end_idx = idx + 1;
                let mut paren_count = 0;

                if tokens[end_idx] == Token::Dot {
                    end_idx = tokens.len() - 1;
                } else {
                    while end_idx < tokens.len() {
                        match tokens[end_idx] {
                            Token::LParen => paren_count += 1,
                            Token::RParen => {
                                paren_count -= 1;
                                if paren_count == 0 {
                                    break;
                                }
                            }
                            Token::Variable(_) => {
                                if paren_count == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        end_idx += 1;
                    }
                }

                // recursively parse the body of the abstraction
                let body = _parse(&tokens[idx + 1..=end_idx])?;
                result.push(Expression::Abstraction(name.clone(), Box::new(body)));
                idx = end_idx;
            }
            Token::Variable(ref variable) => {
                result.push(Expression::Variable(variable.clone()));
            }
            Token::LParen => {
                // keep track of the number of parentheses
                let mut paren_count = 1;
                let mut end_idx = idx + 1;

                // find next parentheses
                while end_idx < tokens.len() {
                    match tokens[end_idx] {
                        Token::LParen => paren_count += 1,
                        Token::RParen => {
                            if paren_count == 1 {
                                // recursively parse the expression inside the parentheses
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
                    return Err(ParseError::UnclosedLParen);
                }
                idx = end_idx;
            }
            // we handle all rparens in the lparen while loop, so if we encounter an rparen here, it is unexpected
            Token::RParen => {
                return Err(ParseError::UnexpectedRParen);
            }
            Token::Dot => {
                result.push(_parse(&tokens[idx + 1..])?);
                idx = tokens.len();
            }
        }
        idx += 1;
    }

    if result.is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    // If the result vector contains only one expression, return the expression
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

/// Display the expression in the normal format
/// (Used by e.g. .to_string() and .print() functions)
impl Display for Expression {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
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
            Expression::Abstraction(name, expr) => {
                write!(fmt, "λ{name}.{expr}")
            }
            /*
            If Variable -> print {name}
            Variable("a")

            Becomes

            a
             */
            Expression::Variable(name) => write!(fmt, "{name}"),
            /*
            If Application
                If lexpr = abs | app -> "({left_expr})"
                Else -> "{left_expr}"
                + " "
                + If rexpr = app | abs -> "({right_expr})"
                Else -> "{right_expr}"
             */
            Expression::Application(left_expr, right_expr) => {
                // Left
                if let Expression::Variable(_) = left_expr.as_ref() {
                    write!(fmt, "{left_expr}")
                } else {
                    write!(fmt, "({left_expr})")
                }?;
                // Separator
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
/// * `idx` - The index of the line the tokens are on (for error printing)
///
/// # Returns
/// The parsed expression
///
/// # Error
/// "Invalid expression [{err_code}] caught during parsing on line {idx}!"
pub(crate) fn parse(tokens: &[Token], idx: usize) -> Expression {
    let expression = _parse(tokens);
    dbg!(&expression);
    match expression {
        // If error in expression, print error and exit
        Err(err_code) => {
            eprintln!(
                "Invalid expression [{}] caught during parsing on line {}!",
                err_code,
                idx + 1
            );

            std::process::exit(1);
        }
        // Else: return the expression
        Ok(expression) => expression,
    }
}

/// Parse the tokens into an expression
/// Only used for benchmarking
/// Unwraps the result, so panics if there is an error, for ultimate speed
pub(crate) fn bench_parse(tokens: &[Token]) -> Expression {
    _parse(tokens).unwrap()
}

/// Parse the tokens into an expression. \
/// If given tokens result in an invalid expression, returns None. \
/// Only used for manual mode, where we want to keep parsing even if there is an error
/// (does not exit with code 1 on error)
pub(crate) fn manual_parse(tokens: &[Token]) -> Option<Expression> {
    let expression = _parse(tokens);
    dbg!(&expression);
    match expression {
        // If error in expression, print error and exit
        Err(err_code) => {
            eprintln!("Invalid expression [{}] caught during parsing!", err_code);
            None
        }
        // Else: reparse the expression (according to the assignment)
        Ok(expression) => expression.into(),
    }
}
