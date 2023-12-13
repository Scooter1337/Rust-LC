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

#[derive(Debug, PartialEq, Eq)]
pub(super) enum LexError {
    EmptyVariableName,
    InvalidCharacter(char),
    InvalidExpression,
    InvalidVariableName,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        dbg!(self);
        match self {
            LexError::EmptyVariableName => write!(f, "Empty variable name"),
            LexError::InvalidVariableName => write!(f, "Invalid variable name"),
            LexError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            LexError::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

pub(super) type Result<T> = std::result::Result<T, LexError>;

pub(super) struct Tokenizer {
    input: String,
}

impl Tokenizer {
    pub(super) fn new(input: String) -> Self {
        Self { input }
    }

    pub(super) fn tokenize(&self) -> Result<Vec<Token>> {
        let mut tokens = Vec::with_capacity(self.input.len());
        let mut chars = self.input.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' | 'λ' => {
                    let mut varname = String::new();
                    for c in chars.by_ref() {
                        match c {
                            '.' | '(' => {
                                if varname.is_empty() {
                                    return Err(LexError::EmptyVariableName);
                                }
                                break;
                            }
                            c if c.is_whitespace() => {
                                if !varname.is_empty() {
                                    // name is not empty, we can break
                                    break;
                                } else {
                                    // name is empty, we can skip
                                    continue;
                                }
                            }
                            c if c.is_alphabetic() => {
                                varname.push(c);
                            }
                            c if c.is_numeric() => {
                                if varname.is_empty() {
                                    return Err(LexError::InvalidVariableName);
                                }
                                varname.push(c);
                            }
                            _ => return Err(LexError::InvalidCharacter(c)),
                        }
                    }
                    tokens.push(Token::Lambda(varname));
                }
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                c if c.is_alphabetic() => {
                    let mut varname = String::from(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() {
                            varname.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Variable(varname));
                }
                c if c.is_whitespace() => (),
                _ => return Err(LexError::InvalidCharacter(c)),
            }
        }
        Ok(tokens)
    }
}
