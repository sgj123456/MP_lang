use crate::lexer::Lexer;

use super::{LexerError, Token, TokenKind};

pub trait TokenProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError>;
}

pub struct WhitespaceProcessor;
impl TokenProcessor for WhitespaceProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        while let Some(' ' | '\t' | '\r') = lexer.peek() {
            lexer.next();
        }
        Ok(None)
    }
}

pub struct UnexpectedCharProcessor;
impl TokenProcessor for UnexpectedCharProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        let Some(c) = lexer.peek() else {
            return Ok(None);
        };
        Err(LexerError::UnexpectedChar(c, span))
    }
}

pub struct NewlineProcessor;
impl TokenProcessor for NewlineProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        if let Some('\n') = lexer.peek() {
            lexer.next();
            Ok(Some(Token {
                kind: TokenKind::Newline,
                span,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct NumberProcessor;
impl TokenProcessor for NumberProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        if let Some(c @ '0'..='9') = lexer.peek() {
            let mut num = String::new();
            num.push(c);
            lexer.next();
            while let Some(c) = lexer.peek() {
                if c.is_ascii_digit() || c == '.' {
                    num.push(c);
                    lexer.next();
                } else {
                    break;
                }
            }
            let num = num
                .parse()
                .map_err(|_| LexerError::InvalidNumber(num.clone(), span))?;
            Ok(Some(Token {
                kind: TokenKind::Number(num),
                span,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct StringProcessor;
impl TokenProcessor for StringProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        if let Some('"') = lexer.peek() {
            lexer.next();
            let mut s = String::new();
            let mut escaped = false;
            let mut closed = false;

            while let Some(c) = lexer.peek() {
                if escaped {
                    match c {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        'r' => s.push('\r'),
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        _ => return Err(LexerError::InvalidEscape(c, span)),
                    }
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    closed = true;
                    lexer.next();
                    break;
                } else {
                    s.push(c);
                }
                lexer.next();
            }

            if !closed {
                return Err(LexerError::UnclosedString(span));
            }

            Ok(Some(Token {
                kind: TokenKind::String(s),
                span,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct OperatorProcessor;
impl TokenProcessor for OperatorProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let Some(c) = lexer.peek() else {
            return Ok(None);
        };
        let span = lexer.span();
        let kind = match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '=' => {
                if let Some('=') = lexer.peek_next() {
                    lexer.next();
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if let Some('=') = lexer.peek_next() {
                    lexer.next();
                    TokenKind::NotEqual
                } else {
                    return Ok(None);
                }
            }
            '>' => {
                if let Some('=') = lexer.peek_next() {
                    lexer.next();
                    TokenKind::GreaterThanOrEqual
                } else {
                    TokenKind::GreaterThan
                }
            }
            '<' => {
                if let Some('=') = lexer.peek_next() {
                    lexer.next();
                    TokenKind::LessThanOrEqual
                } else {
                    TokenKind::LessThan
                }
            }
            _ => return Ok(None),
        };

        lexer.next();
        Ok(Some(Token { kind, span }))
    }
}

pub struct IdentifierProcessor;
impl TokenProcessor for IdentifierProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        let mut ident = String::new();
        while let Some(c @ ('a'..='z' | 'A'..='Z' | '_')) = lexer.peek() {
            ident.push(c);
            lexer.next();
        }
        if ident.is_empty() {
            return Ok(None);
        }
        let kind = match ident.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            _ => TokenKind::Identifier(ident),
        };
        Ok(Some(Token { kind, span }))
    }
}

pub struct SymbolProcessor;
impl TokenProcessor for SymbolProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        let Some(c) = lexer.peek() else {
            return Ok(None);
        };
        let kind = match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            _ => return Ok(None),
        };
        lexer.next();
        Ok(Some(Token { kind, span }))
    }
}

pub struct CommentProcessor;
impl TokenProcessor for CommentProcessor {
    fn process(&self, lexer: &mut Lexer) -> Result<Option<Token>, LexerError> {
        let span = lexer.span();
        if let Some('/') = lexer.peek() {
            if let Some('/') = lexer.peek_next() {
                lexer.next();
                lexer.next();
                let mut comment = String::new();
                while let Some(c) = lexer.peek() {
                    if c == '\n' {
                        break;
                    } else {
                        comment.push(c);
                        lexer.next();
                    }
                }
                Ok(Some(Token {
                    kind: TokenKind::Comment(comment),
                    span,
                }))
            } else if let Some('*') = lexer.peek_next() {
                lexer.next();
                lexer.next();
                let mut comment = String::new();
                let mut closed = false;
                while let Some(c) = lexer.peek() {
                    if c == '*' {
                        if let Some('/') = lexer.peek_next() {
                            lexer.next();
                            lexer.next();
                            closed = true;
                            break;
                        }
                    } else {
                        comment.push(c);
                    }
                    lexer.next();
                }
                if !closed {
                    return Err(LexerError::UnclosedComment(span));
                }
                Ok(Some(Token {
                    kind: TokenKind::Comment(comment),
                    span,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
