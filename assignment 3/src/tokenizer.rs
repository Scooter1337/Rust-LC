// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)
use crate::dbg;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token {
    /// an abstraction with a bound variable
    Lambda(String),
    /// left parenthesis
    LParen,
    /// right parenthesis
    RParen,
    /// a variable with an identifier, lowercase
    LVariable(String),
    /// a variable with an identifier, uppercase
    UVariable(String),
    /// Arrow
    Arrow,
    /// Hat
    Hat,
    /// Colon
    Colon,
    /// Dot
    Dot,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub(super) enum LexError {
    EmptyVariableName(usize),
    InvalidCharacter(char, usize),
    InvalidExpression(usize),
    InvalidVariableName(usize),
    InvalidLambdaVariableChar(char, usize),
    EmptyLambdaVariable(usize),
    InvalidArrow(usize),
    TrailingDot(usize),
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LexError::EmptyVariableName(i) => write!(f, "Empty variable name at pos: {}", i),
            LexError::EmptyLambdaVariable(i) => write!(f, "Empty lambda variable at pos: {}", i),
            LexError::InvalidVariableName(i) => write!(f, "Invalid variable name at pos: {}", i),
            LexError::InvalidCharacter(c, i) => {
                write!(f, "Invalid character: '{}' at pos: {}", c, i)
            }
            LexError::InvalidExpression(i) => write!(f, "Invalid expression at pos: {}", i),
            LexError::InvalidLambdaVariableChar(c, i) => {
                write!(f, "Invalid lambda body character: '{}' at pos: {}", c, i)
            }
            LexError::InvalidArrow(i) => write!(f, "Invalid type arrow at pos: {}", i),
            LexError::TrailingDot(i) => write!(f, "Trailing dot at pos: {}", i),
        }
    }
}

type LexResult<T> = std::result::Result<T, LexError>;

fn _tokenize(input: &str) -> LexResult<Vec<Token>> {
    let mut tokens = Vec::with_capacity(input.len());
    let mut chars = input.chars().enumerate().peekable();

    while let Some((idx, c)) = chars.next() {
        match c {
            '\\' | 'λ' => {
                // retrieve lambda variable
                let mut varname = String::new();
                while let Some((idx, c)) = chars.peek() {
                    match c {
                        // a dot, a lambda and a left parenthesis always signify the end of the variable name
                        '.' | '(' | '\\' | 'λ' | '^' => {
                            if varname.is_empty() {
                                return Err(LexError::EmptyVariableName(*idx + 1));
                            }
                            break;
                        }
                        // a space signifies the end of the variable name if it is not empty
                        c if c.is_whitespace() => {
                            if !varname.is_empty() {
                                // name is not empty, we can break
                                break;
                            } else {
                                // name is empty, we can skip
                                chars.next();
                                continue;
                            }
                        }
                        // The first character of the variable name must be alphabetic, ascii and lowercase
                        c if c.is_ascii_lowercase() => {
                            varname.push(chars.next().unwrap().1);
                        }
                        // The following characters of the variable name must be alphanumeric, but can be unicode
                        c if c.is_alphanumeric() => {
                            if varname.is_empty() {
                                return Err(LexError::InvalidVariableName(*idx + 1));
                            }
                            varname.push(chars.next().unwrap().1);
                        }
                        // All other characters are invalid
                        _ => {
                            let next = chars.next().unwrap();
                            return Err(LexError::InvalidLambdaVariableChar(next.1, next.0 + 1));
                        }
                    }
                }
                if varname.is_empty() {
                    return Err(LexError::EmptyLambdaVariable(idx + 1));
                }
                tokens.push(Token::Lambda(varname));
            }

            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '^' => tokens.push(Token::Hat),
            ':' => tokens.push(Token::Colon),
            '-' => {
                if let Some((_, '>')) = chars.peek() {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    return Err(LexError::InvalidArrow(idx + 1));
                }
            }
            '.' => {
                // check if there is something after the dot
                while let Some((_, c)) = chars.peek() {
                    match c {
                        c if c.is_whitespace() => {
                            // if there is a space, skip it
                            chars.next();
                        }
                        // Colons end the expression in this assignment, therefore we can use that as a limit
                        // if there is a colon, skip it and return the dot
                        ':' => return Err(LexError::TrailingDot(idx + 1)),
                        _ => {
                            tokens.push(Token::Dot);
                            break;
                        }
                    }
                }
            }

            // a variable name must start with an alphabetic ascii character
            c if c.is_ascii_alphabetic() => {
                let mut varname = String::from(c);
                while let Some((_, c)) = chars.peek() {
                    if c.is_alphanumeric() {
                        varname.push(chars.next().unwrap().1);
                    } else {
                        break;
                    }
                }
                if c.is_ascii_lowercase() {
                    tokens.push(Token::LVariable(varname));
                } else {
                    tokens.push(Token::UVariable(varname));
                }
            }

            // ignore whitespace and dots
            c if c.is_whitespace() || c.is_ascii_control() => (),

            // all other characters are invalid
            _ => return Err(LexError::InvalidCharacter(c, idx + 1)),
        }
    }
    Ok(tokens)
}

/// Parse the given string into a vector of tokens
/// If the given string is not a valid expression, prints an error and exits
///
/// # Arguments
/// * `input` - The string to parse
/// * `idx` - The index of the line in the input file (for error reporting)
///
/// # Returns
/// A vector of tokens (Vec<Token>)
///
/// # Error
/// "Invalid expression [{err_code}] caught during tokenizing on line {idx}!"
pub(crate) fn tokenize(input: &str, idx: usize) -> Vec<Token> {
    let tokens = _tokenize(input);
    dbg!(&tokens);
    match tokens {
        // If error in a token, print error and exit
        Err(err_code) => {
            eprintln!(
                "Invalid expression [{}] caught during tokenizing on line {}!",
                err_code,
                idx + 1
            );
            std::process::exit(1);
        }
        // Else: parse the tokens
        Ok(tokens) => tokens,
    }
}

/// Parse the given string into a vector of tokens
/// Only used for benchmarking
/// Unwraps the result, so panics if there is an error, for ultimate speed
pub(crate) fn bench_tokenize(input: &str) -> Vec<Token> {
    _tokenize(input).unwrap()
}

/// Parse the given string into a vector of tokens. \
/// If the given string is not a valid expression, returns None. \
/// Only used for manual mode, where we want to keep parsing even if there is an error
/// (does not exit with code 1 on error)
pub(crate) fn manual_tokenize(input: &str) -> Option<Vec<Token>> {
    let tokens = _tokenize(input);
    dbg!(&tokens);
    match tokens {
        // If error in a token, print error and exit
        Err(err_code) => {
            eprintln!(
                "Invalid expression [{}] caught during tokenizing!",
                err_code
            );
            None
        }

        // Else: parse the tokens
        Ok(tokens) => tokens.into(),
    }
}
