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
    EmptyLambdaVariable(usize),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexError::EmptyVariableName(i) => write!(f, "Empty variable name at idx: {}", i),
            LexError::EmptyLambdaVariable(i) => write!(f, "Empty lambda variable at idx: {}", i),
            LexError::InvalidVariableName(i) => write!(f, "Invalid variable name at idx: {}", i),
            LexError::InvalidCharacter(c, i) => {
                write!(f, "Invalid character: '{}' at idx: {}", c, i)
            }
            LexError::InvalidExpression(i) => write!(f, "Invalid expression at idx: {}", i),
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
                                return Err(LexError::EmptyVariableName(*idx));
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
                                return Err(LexError::InvalidVariableName(*idx));
                            }
                            varname.push(chars.next().unwrap().1);
                        }
                        _ => {
                            let next = chars.next().unwrap();
                            return Err(LexError::InvalidCharacter(next.1, next.0));
                        }
                    }
                }
                if varname.is_empty() {
                    return Err(LexError::EmptyLambdaVariable(idx));
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
            _ => return Err(LexError::InvalidCharacter(c, idx)),
        }
    }
    Ok(tokens)
}

pub(crate) fn tokenize(input: &str, idx: Option<usize>) -> Vec<Token> {
    let tokens = _tokenize(input);
    dbg!(&tokens);
    match tokens {
        // If error in a token, print error and exit
        Err(err_code) => {
            match idx {
                Some(idx) => eprintln!(
                    "Invalid expression '{}' caught during tokenizing on line {}!",
                    err_code,
                    idx + 1
                ),
                None => eprintln!(
                    "Invalid expression '{}' caught during tokenizing!",
                    err_code
                ),
            }
            std::process::exit(1);
        }
        // Else: parse the tokens
        Ok(tokens) => tokens,
    }
}

pub(crate) fn bench_tokenize(input: &str) -> Result<Vec<Token>> {
    _tokenize(input)
}
