mod error;
mod token;

pub use error::LexerError;
pub use token::Span;
pub use token::Token;
pub use token::TokenKind;

mod processors;
use processors::*;

struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.position += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn span(&self) -> Span {
        Span {
            line: self.line,
            column: self.column,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while self.position < self.input.len() {
            for processor in processors() {
                if let Some(token) = processor(self)? {
                    tokens.push(token);
                    break;
                }
            }
            unexpected_char_processor(self)?;
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: self.span(),
        });
        Ok(tokens)
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    Lexer::new(input.to_string()).tokenize()
}
