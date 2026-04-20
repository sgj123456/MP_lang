mod error;
mod token;

use std::str::Chars;

pub use error::LexerError;
pub use error::LexerErrorKind;
pub use token::Span;
pub use token::Token;
pub use token::TokenKind;

struct Cursor<'a> {
    input: Chars<'a>,
    pos: usize,
    line: usize,
    column: usize,
    start_line: usize,
    start_column: usize,
    errors: Vec<LexerError>,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        Cursor {
            input: input.chars(),
            pos: 0,
            line: 1,
            column: 1,
            start_line: 1,
            start_column: 1,
            errors: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.input.next()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.input.clone().nth(n)
    }

    fn bump(&mut self) -> Option<char> {
        self.next()
    }

    fn start_token(&mut self) {
        self.start_line = self.line;
        self.start_column = self.column;
    }

    fn span(&self) -> Span {
        Span {
            line: self.start_line,
            column: self.start_column,
        }
    }

    fn errors(&self) -> &[LexerError] {
        &self.errors
    }

    fn skip_whitespace(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.bump();
                }
                '\n' => {
                    self.start_token();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::Newline,
                        span: self.span(),
                    });
                }
                _ => break,
            }
        }
        None
    }

    fn skip_line_comment(&mut self) -> Option<Token> {
        if self.peek() == Some('/') && self.peek_n(1) == Some('/') {
            self.start_token();
            self.bump();
            self.bump();
            let mut comment = String::new();
            while let Some(c) = self.peek() {
                if c == '\n' {
                    break;
                }
                comment.push(self.bump()?);
            }
            return Some(Token {
                kind: TokenKind::Comment(comment),
                span: self.span(),
            });
        }
        None
    }

    fn skip_block_comment(&mut self) -> Option<Token> {
        if self.peek() == Some('/') && self.peek_n(1) == Some('*') {
            self.start_token();
            self.bump();
            self.bump();
            let mut comment = String::new();
            let mut depth = 1;
            while let Some(c) = self.bump() {
                if c == '/' && self.peek() == Some('*') {
                    self.bump();
                    depth += 1;
                } else if c == '*' && self.peek() == Some('/') {
                    self.bump();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                comment.push(c);
            }
            if depth != 0 {
                self.errors.push(LexerError::new(
                    self.span(),
                    LexerErrorKind::UnclosedComment,
                    "Unclosed block comment".to_string(),
                ));
                return Some(Token {
                    kind: TokenKind::Comment(comment),
                    span: self.span(),
                });
            }
            return Some(Token {
                kind: TokenKind::Comment(comment),
                span: self.span(),
            });
        }
        None
    }

    fn read_number(&mut self) -> Option<Token> {
        if !self.peek()?.is_ascii_digit() {
            return None;
        }

        self.start_token();
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num_str.push(self.bump()?);
            } else if c == '.' && !has_dot {
                has_dot = true;
                num_str.push(self.bump()?);
            } else {
                break;
            }
        }

        let kind = TokenKind::Number(num_str.parse().ok()?);

        Some(Token {
            kind,
            span: self.span(),
        })
    }

    fn read_string(&mut self) -> Option<Token> {
        if self.peek() != Some('"') {
            return None;
        }

        self.start_token();
        self.bump();
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c == '"' {
                self.bump();
                return Some(Token {
                    kind: TokenKind::String(s),
                    span: self.span(),
                });
            } else if c == '\\' {
                self.bump();
                match self.peek() {
                    Some('n') => {
                        s.push('\n');
                        self.bump();
                    }
                    Some('t') => {
                        s.push('\t');
                        self.bump();
                    }
                    Some('r') => {
                        s.push('\r');
                        self.bump();
                    }
                    Some('\\') => {
                        s.push('\\');
                        self.bump();
                    }
                    Some('"') => {
                        s.push('"');
                        self.bump();
                    }
                    Some(c) => {
                        s.push('\\');
                        s.push(c);
                        self.bump();
                    }
                    None => {
                        self.errors.push(LexerError::new(
                            self.span(),
                            LexerErrorKind::UnclosedString,
                            "Unclosed string".to_string(),
                        ));
                        return Some(Token {
                            kind: TokenKind::String(s),
                            span: self.span(),
                        });
                    }
                }
            } else if c == '\n' {
                self.errors.push(LexerError::new(
                    self.span(),
                    LexerErrorKind::UnclosedString,
                    "Unclosed string".to_string(),
                ));
                return Some(Token {
                    kind: TokenKind::String(s),
                    span: self.span(),
                });
            } else {
                s.push(self.bump()?);
            }
        }

        self.errors.push(LexerError::new(
            self.span(),
            LexerErrorKind::UnclosedString,
            "Unclosed string".to_string(),
        ));
        Some(Token {
            kind: TokenKind::String(s),
            span: self.span(),
        })
    }

    fn read_identifier(&mut self) -> Option<Token> {
        if !self.peek()?.is_alphabetic() && self.peek() != Some('_') {
            return None;
        }

        self.start_token();
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.bump()?);
            } else {
                break;
            }
        }

        let kind = match ident.as_str() {
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier(ident),
        };

        Some(Token {
            kind,
            span: self.span(),
        })
    }

    fn read_punct(&mut self) -> Option<Token> {
        let c = self.peek()?;
        self.start_token();
        let kind = match c {
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '&' => {
                if self.peek_n(1) == Some('&') {
                    self.bump();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::LogicalAnd,
                        span: self.span(),
                    });
                }
                return None;
            }
            '|' => {
                if self.peek_n(1) == Some('|') {
                    self.bump();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::LogicalOr,
                        span: self.span(),
                    });
                }
                return None;
            }
            '=' => {
                if self.peek_n(1) == Some('=') {
                    self.bump();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::Equal,
                        span: self.span(),
                    });
                }
                TokenKind::Assign
            }
            '!' => {
                if self.peek_n(1) == Some('=') {
                    self.bump();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::NotEqual,
                        span: self.span(),
                    });
                }
                self.bump();
                return Some(Token {
                    kind: TokenKind::Not,
                    span: self.span(),
                });
            }
            '>' => {
                if self.peek_n(1) == Some('=') {
                    self.bump();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::GreaterThanOrEqual,
                        span: self.span(),
                    });
                }
                TokenKind::GreaterThan
            }
            '<' => {
                if self.peek_n(1) == Some('=') {
                    self.bump();
                    self.bump();
                    return Some(Token {
                        kind: TokenKind::LessThanOrEqual,
                        span: self.span(),
                    });
                }
                TokenKind::LessThan
            }
            ':' => TokenKind::Colon,
            _ => return None,
        };
        self.bump();
        Some(Token {
            kind,
            span: self.span(),
        })
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut cursor = Cursor::new(input);
    let mut tokens = Vec::new();

    while cursor.peek().is_some() {
        cursor.start_token();

        if let Some(newline_token) = cursor.skip_whitespace() {
            tokens.push(newline_token);
            continue;
        }

        if cursor.peek().is_none() {
            break;
        }

        if let Some(comment) = cursor.skip_line_comment() {
            tokens.push(comment);
            continue;
        }

        if let Some(token) = cursor.skip_block_comment() {
            tokens.push(token);
            continue;
        }

        if let Some(token) = cursor.read_number() {
            tokens.push(token);
            continue;
        }

        if let Some(token) = cursor.read_string() {
            tokens.push(token);
            continue;
        }

        if let Some(token) = cursor.read_identifier() {
            tokens.push(token);
            continue;
        }

        if let Some(token) = cursor.read_punct() {
            tokens.push(token);
            continue;
        }

        let c = cursor.peek().unwrap();
        cursor.errors.push(LexerError::new(
            cursor.span(),
            LexerErrorKind::UnexpectedCharacter(c),
            format!("Unexpected character: '{}'", c),
        ));
        cursor.bump();
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: cursor.span(),
    });

    if cursor.errors().is_empty() {
        Ok(tokens)
    } else {
        Err(cursor.errors()[0].clone())
    }
}
