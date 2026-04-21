use crate::lexer::{TokenKind, tokenize};
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
        let tokens = tokenize(content).ok()?;

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
            TokenKind::Identifier(name) => {
                let (content, _) = self.get_identifier_info(name);
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(content)),
                    range: None,
                })
            }
            TokenKind::Number(n) => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        format!("**Number**: {}", n)
                    )),
                    range: None,
                })
            }
            TokenKind::Boolean(b) => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        format!("**Boolean**: {}", b)
                    )),
                    range: None,
                })
            }
            TokenKind::String(s) => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        format!("**String**: \"{}\"", s)
                    )),
                    range: None,
                })
            }
            TokenKind::Let => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**let** - Variable declaration keyword\n\nUsed to declare a new variable.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Fn => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**fn** - Function definition keyword\n\nUsed to define a new function.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::If => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**if** - Conditional statement keyword\n\nUsed for conditional execution.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Else => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**else** - Else branch keyword\n\nUsed as the alternative branch of an if statement.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::While => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**while** - Loop statement keyword\n\nUsed to create a loop that executes while a condition is true.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Return => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**return** - Return statement keyword\n\nUsed to exit a function and optionally return a value.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Break => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**break** - Break statement keyword\n\nUsed to exit a loop immediately.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Continue => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**continue** - Continue statement keyword\n\nUsed to skip to the next iteration of a loop.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Plus => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**+** - Addition operator\n\nAdds two numbers or concatenates two strings.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Minus => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**-** - Subtraction operator\n\nSubtracts the right operand from the left operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Multiply => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**\\*** - Multiplication operator\n\nMultiplies two numbers.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Divide => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**/** - Division operator\n\nDivides the left operand by the right operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Modulo => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**%** - Modulo operator\n\nReturns the remainder of the division of the left operand by the right operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Equal => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**==** - Equality comparison operator\n\nReturns true if both operands are equal.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::NotEqual => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**!=** - Inequality comparison operator\n\nReturns true if operands are not equal.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::GreaterThan => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**>** - Greater than operator\n\nReturns true if left operand is greater than right operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::LessThan => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**<** - Less than operator\n\nReturns true if left operand is less than right operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::GreaterThanOrEqual => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**>=** - Greater than or equal operator\n\nReturns true if left operand is greater than or equal to right operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::LessThanOrEqual => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**<=** - Less than or equal operator\n\nReturns true if left operand is less than or equal to right operand.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Assign => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**=** - Assignment operator\n\nAssigns a value to a variable.".to_string()
                    )),
                    range: None,
                })
            }
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn get_identifier_info(&self, name: &str) -> (String, &'static str) {
        if self.is_builtin_keyword(name) {
            return (format!("**{}**", name), "keyword");
        }

        if let Some(builtin_info) = self.get_builtin_function_info(name) {
            return builtin_info;
        }

        (
            format!("**{}**\n\nIdentifier (variable or function)", name),
            "identifier",
        )
    }

    fn is_builtin_keyword(&self, name: &str) -> bool {
        matches!(name, "true" | "false" | "nil")
    }

    fn get_builtin_function_info(&self, name: &str) -> Option<(String, &'static str)> {
        match name {
            "print" => Some((
                "**print(expr)**\n\nBuilt-in function that prints the value of expr to the console.".to_string(),
                "function"
            )),
            "input" => Some((
                "**input()**\n\nBuilt-in function that reads a string from the console.".to_string(),
                "function"
            )),
            "len" => Some((
                "**len(str)**\n\nBuilt-in function that returns the length of str (string, array, or object).".to_string(),
                "function"
            )),
            "type" => Some((
                "**type(expr)**\n\nBuilt-in function that returns the type of expr as a string.".to_string(),
                "function"
            )),
            "str" => Some((
                "**str(num)**\n\nBuilt-in function that converts num to a string.".to_string(),
                "function"
            )),
            "int" => Some((
                "**int(str)**\n\nBuilt-in function that converts str to an integer.".to_string(),
                "function"
            )),
            "float" => Some((
                "**float(str)**\n\nBuilt-in function that converts str to a float.".to_string(),
                "function"
            )),
            "random" => Some((
                "**random() | random(max) | random(min, max)**\n\nBuilt-in function that generates a random number.".to_string(),
                "function"
            )),
            "push" => Some((
                "**push(array, item)**\n\nBuilt-in function that adds an item to an array.".to_string(),
                "function"
            )),
            "pop" => Some((
                "**pop(array)**\n\nBuilt-in function that removes and returns the last item from an array.".to_string(),
                "function"
            )),
            "time" => Some((
                "**time()**\n\nBuilt-in function that returns the current Unix timestamp in seconds.".to_string(),
                "function"
            )),
            _ => None,
        }
    }
}
