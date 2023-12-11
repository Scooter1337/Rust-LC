pub enum Token {
    Lambda,
    Variable(String),
    Dot,
    OpenParen,
    CloseParen,
}

struct Tokenizer {
    input: String,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' | 'λ' => tokens.push(Token::Lambda),
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                c if c.is_alphabetic() => {
                    let mut var = String::new();
                    var.push(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphabetic() {
                            var.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Variable(var));
                }
                _ => (),
            }
        }
        tokens
    }
}
