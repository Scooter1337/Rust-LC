// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)
use crate::dbg;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token {
    /// an abstraction with a bound variable
    Lambda(String),
    /// left parenthesis
    LParen,
    /// right parenthesis
    RParen,
    /// a variable with an identifier
    Variable(String),
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
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        }
    }
}

type Result<T> = std::result::Result<T, LexError>;

fn _tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::with_capacity(input.len());
    let mut chars = input.chars().enumerate().peekable();

    while let Some((idx, c)) = chars.next() {
        match c {
            '\\' | 'λ' => {
                let mut varname = String::new();
                while let Some((idx, c)) = chars.peek() {
                    match c {
                        '.' | '(' => {
                            if varname.is_empty() {
                                return Err(LexError::EmptyVariableName(*idx + 1));
                            }
                            break;
                        }
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
                        c if c.is_ascii_alphabetic() => {
                            varname.push(chars.next().unwrap().1);
                        }
                        c if c.is_alphanumeric() => {
                            if varname.is_empty() {
                                return Err(LexError::InvalidVariableName(*idx + 1));
                            }
                            varname.push(chars.next().unwrap().1);
                        }
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
            c if c.is_ascii_alphabetic() => {
                let mut varname = String::from(c);
                while let Some((_, c)) = chars.peek() {
                    if c.is_alphanumeric() {
                        varname.push(chars.next().unwrap().1);
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Variable(varname));
            }
            c if c.is_whitespace() || c == '.' => (),
            _ => return Err(LexError::InvalidCharacter(c, idx + 1)),
        }
    }
    Ok(tokens)
}

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

pub(crate) fn bench_tokenize(input: &str) -> Vec<Token> {
    _tokenize(input).unwrap()
}

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
