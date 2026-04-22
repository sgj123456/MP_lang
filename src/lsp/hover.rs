use crate::lexer::TokenKind;
use tower_lsp_server::ls_types::*;

#[derive(Debug)]
pub struct MpHover;

impl Default for MpHover {
    fn default() -> Self {
        Self::new()
    }
}

impl MpHover {
    pub fn new() -> Self {
        Self
    }

    pub fn hover(&self, content: &str, position: Position) -> Option<Hover> {
        let tokens = crate::lexer::tokenize(content);

        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        for token in &tokens {
            let token_end_col = token.span.column + token.kind.to_string().len();
            if token.span.line == line && token.span.column <= col && token_end_col > col {
                return self.get_hover_for_token(token);
            }
        }

        for token in tokens.iter().rev() {
            if token.span.line == line && token.span.column <= col {
                return self.get_hover_for_token(token);
            }
        }

        None
    }

    fn get_hover_for_token(&self, token: &crate::lexer::Token) -> Option<Hover> {
        match &token.kind {
            TokenKind::Identifier(name) => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(format!("**{}**", name))),
                range: None,
            }),
            TokenKind::Number(n) => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(format!("**Number**: {}", n))),
                range: None,
            }),
            TokenKind::Boolean(b) => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(format!(
                    "**Boolean**: {}",
                    b
                ))),
                range: None,
            }),
            TokenKind::String(s) => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(format!(
                    "**String**: \"{}\"",
                    s
                ))),
                range: None,
            }),
            TokenKind::Let => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**let** - Variable declaration keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Fn => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**fn** - Function definition keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::If => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**if** - Conditional statement keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Else => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**else** - Else branch keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::While => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**while** - Loop statement keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Return => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**return** - Return statement keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Break => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**break** - Break statement keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Continue => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**continue** - Continue statement keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Struct => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**struct** - Struct definition keyword".to_string(),
                )),
                range: None,
            }),
            TokenKind::Not => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**!** - Logical NOT operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Plus => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**+** - Addition operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Minus => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**-** - Subtraction operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Multiply => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "***** - Multiplication operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Divide => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**/** - Division operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Modulo => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**%** - Modulo operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Equal => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**==** - Equality comparison operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::NotEqual => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**!=** - Inequality comparison operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::GreaterThan => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**>** - Greater than operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::LessThan => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**<** - Less than operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::GreaterThanOrEqual => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**>=** - Greater than or equal operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::LessThanOrEqual => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**<=** - Less than or equal operator".to_string(),
                )),
                range: None,
            }),
            TokenKind::Assign => Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "**=** - Assignment operator".to_string(),
                )),
                range: None,
            }),
            _ => None,
        }
    }
}
