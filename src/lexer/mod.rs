pub mod error;
pub mod token;

pub use error::LexerError;
pub use token::Span;
pub use token::Token;
pub use token::TokenKind;

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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let c = self.input[self.position];
            let span = Span {
                line: self.line,
                column: self.column,
            };

            match c {
                ' ' | '\t' | '\r' => {
                    self.position += 1;
                    self.column += 1;
                }
                '\n' => {
                    tokens.push(Token {
                        kind: TokenKind::Newline,
                        span,
                    });
                    self.position += 1;
                    self.line += 1;
                    self.column = 1;
                }
                '+' => {
                    tokens.push(Token {
                        kind: TokenKind::Plus,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                '-' => {
                    tokens.push(Token {
                        kind: TokenKind::Minus,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                '*' => {
                    tokens.push(Token {
                        kind: TokenKind::Multiply,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                '/' => {
                    self.position += 1;
                    self.column += 1;
                    if self.position < self.input.len() {
                        match self.input[self.position] {
                            '/' => {
                                // 处理单行注释
                                let mut comment = String::new();
                                while self.position < self.input.len()
                                    && self.input[self.position] != '\n'
                                {
                                    comment.push(self.input[self.position]);
                                    self.position += 1;
                                }
                                tokens.push(Token {
                                    kind: TokenKind::Comment(comment),
                                    span,
                                });
                            }
                            '*' => {
                                // 处理多行注释
                                self.position += 1;
                                self.column += 1;
                                let mut comment = String::new();
                                let mut depth = 1;
                                while depth > 0 && self.position + 1 < self.input.len() {
                                    if self.input[self.position] == '*'
                                        && self.input[self.position + 1] == '/'
                                    {
                                        depth -= 1;
                                        self.position += 2;
                                        self.column += 2;
                                        if depth == 0 {
                                            break;
                                        }
                                    } else if self.input[self.position] == '/'
                                        && self.input[self.position + 1] == '*'
                                    {
                                        depth += 1;
                                        self.position += 2;
                                        self.column += 2;
                                    } else {
                                        if self.input[self.position] == '\n' {
                                            self.line += 1;
                                            self.column = 1;
                                        } else {
                                            self.column += 1;
                                        }
                                        comment.push(self.input[self.position]);
                                        self.position += 1;
                                    }
                                }
                                tokens.push(Token {
                                    kind: TokenKind::Comment(comment),
                                    span,
                                });
                            }
                            _ => {
                                tokens.push(Token {
                                    kind: TokenKind::Divide,
                                    span,
                                });
                            }
                        }
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Divide,
                            span,
                        });
                    }
                }
                '=' => {
                    self.position += 1;
                    self.column += 1;
                    if self.position < self.input.len() && self.input[self.position] == '=' {
                        self.position += 1;
                        self.column += 1;
                        tokens.push(Token {
                            kind: TokenKind::Equal,
                            span,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Equal,
                            span,
                        });
                    }
                }
                '!' => {
                    self.position += 1;
                    self.column += 1;
                    if self.position < self.input.len() && self.input[self.position] == '=' {
                        self.position += 1;
                        self.column += 1;
                        tokens.push(Token {
                            kind: TokenKind::NotEqual,
                            span,
                        });
                    } else {
                        return Err(LexerError::UnknownOperator('!', span));
                    }
                }
                '>' => {
                    self.position += 1;
                    self.column += 1;
                    if self.position < self.input.len() && self.input[self.position] == '=' {
                        self.position += 1;
                        self.column += 1;
                        tokens.push(Token {
                            kind: TokenKind::GreaterThanOrEqual,
                            span,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::GreaterThan,
                            span,
                        });
                    }
                }
                '<' => {
                    self.position += 1;
                    self.column += 1;
                    if self.position < self.input.len() && self.input[self.position] == '=' {
                        self.position += 1;
                        self.column += 1;
                        tokens.push(Token {
                            kind: TokenKind::LessThanOrEqual,
                            span,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::LessThan,
                            span,
                        });
                    }
                }
                '(' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftParen,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::RightParen,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                ',' => {
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                '{' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftBrace,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                '}' => {
                    tokens.push(Token {
                        kind: TokenKind::RightBrace,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        span,
                    });
                    self.position += 1;
                    self.column += 1;
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    while self.position < self.input.len() {
                        let c = self.input[self.position];
                        if c.is_ascii_digit() || c == '.' {
                            num_str.push(c);
                            self.position += 1;
                            self.column += 1;
                        } else {
                            break;
                        }
                    }
                    match num_str.parse::<f64>() {
                        Ok(num) => {
                            tokens.push(Token {
                                kind: TokenKind::Number(num),
                                span,
                            });
                        }
                        Err(_) => {
                            return Err(LexerError::InvalidNumber(num_str, span));
                        }
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    while self.position < self.input.len() {
                        let c = self.input[self.position];
                        if c.is_ascii_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.position += 1;
                            self.column += 1;
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "true" => tokens.push(Token {
                            kind: TokenKind::Boolean(true),
                            span,
                        }),
                        "false" => tokens.push(Token {
                            kind: TokenKind::Boolean(false),
                            span,
                        }),
                        "let" => tokens.push(Token {
                            kind: TokenKind::Let,
                            span,
                        }),
                        "fn" => tokens.push(Token {
                            kind: TokenKind::Fn,
                            span,
                        }),
                        "if" => tokens.push(Token {
                            kind: TokenKind::If,
                            span,
                        }),
                        "else" => tokens.push(Token {
                            kind: TokenKind::Else,
                            span,
                        }),
                        "while" => tokens.push(Token {
                            kind: TokenKind::While,
                            span,
                        }),
                        "return" => tokens.push(Token {
                            kind: TokenKind::Return,
                            span,
                        }),
                        _ => tokens.push(Token {
                            kind: TokenKind::Identifier(ident),
                            span,
                        }),
                    }
                }
                '"' => {
                    self.position += 1; // 跳过开始引号
                    self.column += 1;
                    let mut s = String::new();
                    let mut escaped = false;
                    let mut closed = false;

                    while self.position < self.input.len() {
                        let c = self.input[self.position];
                        if escaped {
                            match c {
                                'n' => s.push('\n'),
                                't' => s.push('\t'),
                                'r' => s.push('\r'),
                                '"' => s.push('"'),
                                '\\' => s.push('\\'),
                                _ => s.push(c),
                            }
                            escaped = false;
                            self.position += 1;
                            self.column += 1;
                        } else if c == '\\' {
                            escaped = true;
                            self.position += 1;
                            self.column += 1;
                        } else if c == '"' {
                            closed = true;
                            self.position += 1;
                            self.column += 1;
                            break;
                        } else {
                            s.push(c);
                            self.position += 1;
                            self.column += 1;
                        }
                    }

                    if !closed {
                        return Err(LexerError::UnexpectedChar('"', span));
                    }

                    tokens.push(Token {
                        kind: TokenKind::String(s),
                        span,
                    });
                }
                _ => {
                    return Err(LexerError::UnexpectedChar(c, span));
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span {
                line: self.line,
                column: self.column,
            },
        });
        Ok(tokens)
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    Lexer::new(input.to_string()).tokenize()
}
