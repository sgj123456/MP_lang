mod error;
mod token;

pub use error::LexerError;
pub use token::Span;
pub use token::Token;
pub use token::TokenKind;

mod processors;
use processors::*;

struct PositionTracker {
    line: usize,
    column: usize,
}

impl PositionTracker {
    fn new() -> Self {
        PositionTracker { line: 1, column: 1 }
    }

    fn advance(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
        } else {
            self.column += 1;
        }
    }

    fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    pos_tracker: PositionTracker,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            pos_tracker: PositionTracker::new(),
        }
    }

    fn processors() -> [Box<dyn TokenProcessor>; 9] {
        [
            Box::new(WhitespaceProcessor),
            Box::new(NewlineProcessor),
            Box::new(NumberProcessor),
            Box::new(StringProcessor),
            Box::new(CommentProcessor),
            Box::new(OperatorProcessor),
            Box::new(IdentifierProcessor),
            Box::new(SymbolProcessor),
            Box::new(UnexpectedCharProcessor),
        ]
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(c) = c {
            self.position += 1;
            self.pos_tracker.advance(c);
        }
        c
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).cloned()
    }

    fn span(&self) -> Span {
        Span {
            line: self.pos_tracker.line,
            column: self.pos_tracker.column,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while self.position < self.input.len() {
            for processor in Self::processors() {
                if let Some(token) = processor.process(self)? {
                    tokens.push(token);
                    break;
                }
            }
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
