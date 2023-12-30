// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)
use crate::dbg;
use crate::tokenizer::Token;

use std::fmt::{Display, Formatter, Result};

// Boxes are heap allocated, so we can use them to store the expression tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Expression {
    /// <Expression> <Expression>
    Application(Box<Expression>, Box<Expression>),
    /// <Variable>
    Variable(String),
    /// \ <Variable>^<Type> . <Expression>
    Abstraction(String, Box<Type>, Box<Expression>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Type {
    /// <Type> -> <Type>
    Function(Box<Type>, Box<Type>),
    /// <Variable>
    Variable(String),
}

#[derive(Debug, Clone)]
pub(crate) enum Judgement {
    /// <Expression> : <Type>
    Judgement(Box<Expression>, Box<Type>),
}

#[derive(Debug)]
enum ParseError {
    EmptyExpression,
    InvalidExpression,
    UnexpectedRParen,
    UnclosedLParen,
    NoAbstractionBody,
    NoTypeHat,
    InvalidType,
    ArrowBeforeType,
    NoType,
    TypeSyntaxOutsideType,
    TooManyColons,
    JudgementTooShort,
    EmptyJudgement,
    ExprSyntaxOutsideExpr,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
            ParseError::UnexpectedRParen => write!(f, "Unexpected right parenthesis"),
            ParseError::UnclosedLParen => write!(f, "Unclosed left parenthesis"),
            ParseError::NoAbstractionBody => write!(f, "Missing abstraction body"),
            ParseError::NoTypeHat => write!(f, "Missing type hat"),
            ParseError::InvalidType => write!(f, "Invalid type"),
            ParseError::ArrowBeforeType => write!(f, "Arrow before type"),
            ParseError::NoType => write!(f, "Missing type"),
            ParseError::TypeSyntaxOutsideType => write!(f, "Type syntax outside type found"),
            ParseError::TooManyColons => write!(f, "Too many colons"),
            ParseError::JudgementTooShort => {
                write!(f, "Judgement too short. Should be at least 3 tokens long")
            }
            ParseError::EmptyJudgement => write!(f, "Empty judgement"),
            ParseError::ExprSyntaxOutsideExpr => write!(f, "Expression syntax outside expression"),
        }
    }
}

type ParseResult<T> = std::result::Result<T, ParseError>;

fn _parse_type(tokens: &[Token]) -> ParseResult<Type> {
    let mut idx = 0;
    let mut result: Vec<Type> = Vec::new();

    while idx < tokens.len() {
        match &tokens[idx] {
            Token::UVariable(ref var) => {
                result.push(Type::Variable(var.clone()));
            }
            Token::Arrow => {
                if result.is_empty() {
                    return Err(ParseError::ArrowBeforeType);
                }
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
                                // recursively parse the type inside the parentheses
                                result.push(_parse_type(&tokens[(idx + 1)..end_idx])?);
                            }
                            paren_count -= 1;
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
            Token::RParen => return Err(ParseError::UnexpectedRParen),
            _ => return Err(ParseError::ExprSyntaxOutsideExpr),
        }
        idx += 1;
    }
    if result.is_empty() {
        return Err(ParseError::NoType);
    }
    if result.len() == 1 {
        Ok(result.pop().unwrap())
    } else {
        match result.into_iter().reduce(|left_type, right_type| {
            Type::Function(Box::new(left_type), Box::new(right_type))
        }) {
            Some(typ) => Ok(typ),
            None => Err(ParseError::InvalidType),
        }
    }
}

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

                if tokens[idx + 1] != Token::Hat {
                    return Err(ParseError::NoTypeHat);
                }

                // find the type
                let mut end_idx = idx + 2;
                let mut paren_count = 0;

                while end_idx < tokens.len() {
                    match tokens[end_idx] {
                        Token::LParen => paren_count += 1,
                        Token::RParen => paren_count -= 1,
                        Token::Arrow => {
                            if paren_count == 0 {
                                return Err(ParseError::InvalidType);
                            }
                        }
                        _ => {}
                    }
                    if paren_count == 0 {
                        break;
                    }
                    end_idx += 1;
                }

                let abstype = _parse_type(&tokens[idx + 2..end_idx + 1])?;
                idx = end_idx;

                // 28 DEC edit: precedence rules are other way around. λx.a b = λx.(a) b and not λx.(a b)
                // Therefore, we find the end of the abstraction body first, and then recursively parse the body
                let mut end_idx = idx + 1;
                let mut paren_count = 0;

                // we need to account for the type, so just looking at parentheses is not enough
                // we set body to true once we have found a LVariable
                if tokens[end_idx] == Token::Dot {
                    end_idx = tokens.len() - 1;
                } else {
                    let mut body = false;
                    while end_idx < tokens.len() {
                        match tokens[end_idx] {
                            Token::LParen => paren_count += 1,
                            Token::RParen => {
                                paren_count -= 1;
                                if paren_count == 0 && body {
                                    break;
                                }
                            }
                            Token::LVariable(_) => {
                                if paren_count == 0 {
                                    break;
                                }
                                body = true;
                            }
                            _ => {}
                        }
                        end_idx += 1;
                    }
                }
                if end_idx >= tokens.len() {
                    return Err(ParseError::NoAbstractionBody);
                }

                // recursively parse the body of the abstraction
                let body = _parse(&tokens[idx + 1..=end_idx])?;
                result.push(Expression::Abstraction(
                    name.clone(),
                    Box::new(abstype),
                    Box::new(body),
                ));
                idx = end_idx;
            }
            Token::LVariable(ref variable) => {
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
                                result.push(_parse(&tokens[(idx + 1)..end_idx])?);
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
            Token::RParen => return Err(ParseError::UnexpectedRParen),
            Token::Colon => {
                break;
            }
            Token::Arrow | Token::Hat | Token::UVariable(_) => {
                return Err(ParseError::TypeSyntaxOutsideType);
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
            Expression::Abstraction(name, typ, expr) => {
                write!(fmt, "λ{name}^{typ}.{expr}")
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
                If lexpr = abs -> "({left_expr})"
                Else -> "{left_expr}"
                + " "
                + If rexpr = app | abs -> "({right_expr})"
                | Else -> "{right_expr}"
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
                    Expression::Application(_, _) | Expression::Abstraction(_, _, _) => {
                        write!(fmt, "({right_expr})")
                    }

                    r_expr => write!(fmt, "{r_expr}"),
                }
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            Type::Function(left_type, right_type) => {
                write!(fmt, "({left_type} -> {right_type})")
            }
            Type::Variable(name) => write!(fmt, "{name}"),
        }
    }
}

impl Display for Judgement {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            Judgement::Judgement(expr, typ) => write!(fmt, "({expr}) : {typ}"),
        }
    }
}

fn judgement(tokens: &[Token]) -> ParseResult<Judgement> {
    if tokens.len() < 3 {
        return Err(ParseError::JudgementTooShort);
    }
    let split = tokens.split(|t| *t == Token::Colon).collect::<Vec<_>>();
    match split.len() {
        0 => Err(ParseError::EmptyJudgement),
        1 => {
            if !tokens.contains(&Token::Colon) {
                return Err(ParseError::NoType);
            }
            if tokens[0] == Token::Colon {
                return Err(ParseError::EmptyExpression);
            }
            Err(ParseError::NoType)
        }
        2 => {
            let expr = _parse(split[0])?;
            let typ = _parse_type(split[1])?;
            Ok(Judgement::Judgement(Box::new(expr), Box::new(typ)))
        }
        _ => Err(ParseError::TooManyColons),
    }
}

/// Parse the tokens into a judgement
/// If given tokens result in an invalid judgement, prints an error and exits the program
///
/// # Arguments
/// * `tokens` - The tokens to parse
/// * `idx` - The index of the line the tokens are on (for error printing)
///
/// # Returns
/// The parsed judgement
///
/// # Error
/// "Invalid judgement [{err_code}] caught during parsing on line {idx}!"
pub(crate) fn parse(tokens: &[Token], idx: usize) -> Judgement {
    let judgement = judgement(tokens);
    dbg!(&judgement);
    match judgement {
        // If error in judgement, print error and exit
        Err(err_code) => {
            eprintln!(
                "Invalid judgement [{}] caught during parsing on line {}!",
                err_code,
                idx + 1
            );

            std::process::exit(1);
        }
        // Else: return the judgement
        Ok(judgement) => judgement,
    }
}

/// Parse the tokens into a judgement
/// Only used for benchmarking
/// Unwraps the result, so panics if there is an error, for ultimate speed
pub(crate) fn bench_parse(tokens: &[Token]) -> Judgement {
    judgement(tokens).unwrap()
}

/// Parse the tokens into a judgement. \
/// If given tokens result in an invalid judgement, returns None. \
/// Only used for manual mode, where we want to keep parsing even if there is an error
/// (does not exit with code 1 on error)
pub(crate) fn manual_parse(tokens: &[Token]) -> Option<Judgement> {
    let judgement = judgement(tokens);
    dbg!(&judgement);
    match judgement {
        // If error in judgement, print error and exit
        Err(err_code) => {
            eprintln!("Invalid judgement [{}] caught during parsing!", err_code);
            None
        }
        // Else: return the judgement
        Ok(judgement) => judgement.into(),
    }
}
