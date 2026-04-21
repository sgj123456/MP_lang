use crate::lexer::{TokenKind, tokenize, tokenize_with_errors};
use crate::parser::{parse, StmtKind, Expr, ExprKind};
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
        let tokens = tokenize(content);

        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        for token in &tokens {
            let token_end_col = token.span.column + token.kind.to_string().len();
            if token.span.line == line && token.span.column <= col && token_end_col > col {
                return self.get_hover_for_token(token, content);
            }
        }

        for token in tokens.iter().rev() {
            if token.span.line == line && token.span.column <= col {
                return self.get_hover_for_token(token, content);
            }
        }

        None
    }

    fn get_hover_for_token(&self, token: &crate::lexer::Token, content: &str) -> Option<Hover> {
        match &token.kind {
            TokenKind::Identifier(name) => {
                let (content, _) = self.get_identifier_info(name, content);
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
            TokenKind::Struct => {
                Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        "**struct** - Struct definition keyword\n\nUsed to define a custom data type with named fields.".to_string()
                    )),
                    range: None,
                })
            }
            TokenKind::Not => {
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
    fn get_identifier_info(&self, name: &str, content: &str) -> (String, &'static str) {
        if self.is_builtin_keyword(name) {
            return (format!("**{}**", name), "keyword");
        }

        if let Some(builtin_info) = self.get_builtin_function_info(name) {
            return builtin_info;
        }

        let (tokens, _) = tokenize_with_errors(content);
        let ast = parse(tokens);

        for stmt in &ast {
            if let StmtKind::Struct { name: struct_name, fields } = &stmt.kind {
                if struct_name == name {
                    let fields_str: Vec<String> = fields
                        .iter()
                        .map(|(field_name, default_value)| {
                            if let Some(expr) = default_value {
                                let type_str = self.infer_expr_type(expr);
                                format!("{}: {}", field_name, type_str)
                            } else {
                                format!("{}", field_name)
                            }
                        })
                        .collect();
                    return (
                        format!("**struct {}** {{\n  {}\n}}", name, fields_str.join(",\n  ")),
                        "struct",
                    );
                }
            }

            if let StmtKind::Function { name: func_name, params, body } = &stmt.kind {
                if func_name == name {
                    let return_type = self.infer_expr_type(body);
                    let params_str = params.join(", ");
                    return (
                        format!("**fn {}({}) -> {}**", name, params_str, return_type),
                        "function",
                    );
                }
            }

            if let StmtKind::Let { name: var_name, value, .. } = &stmt.kind {
                if var_name == name {
                    let var_type = self.infer_expr_type(value);
                    return (format!("**{}**: {}", name, var_type), "variable");
                }
            }
        }

        (
            format!("**{}**\n\nIdentifier (variable or function)", name),
            "identifier",
        )
    }

    fn infer_expr_type(&self, expr: &Expr) -> String {
        match &expr.kind {
            ExprKind::Number(n) => match n {
                crate::runtime::environment::value::Number::Int(_) => "int".to_string(),
                crate::runtime::environment::value::Number::Float(_) => "float".to_string(),
            },
            ExprKind::Boolean(_) => "bool".to_string(),
            ExprKind::String(_) => "string".to_string(),
            ExprKind::Array(_) => "array".to_string(),
            ExprKind::Object(_) => "object".to_string(),
            ExprKind::FunctionCall { name, .. } => self.get_builtin_return_type(name),
            ExprKind::Variable(_) => "unknown".to_string(),
            ExprKind::If { .. } => "unknown".to_string(),
            ExprKind::While { .. } => "unknown".to_string(),
            ExprKind::Block(_) => "unknown".to_string(),
            ExprKind::Index { .. } => "unknown".to_string(),
            ExprKind::GetProperty { .. } => "unknown".to_string(),
            ExprKind::UnaryOp { .. } => "unknown".to_string(),
            ExprKind::BinaryOp { op, .. } => {
                if matches!(op, TokenKind::Equal | TokenKind::NotEqual | TokenKind::GreaterThan 
                    | TokenKind::LessThan | TokenKind::GreaterThanOrEqual | TokenKind::LessThanOrEqual) {
                    "bool".to_string()
                } else {
                    "number".to_string()
                }
            }
            ExprKind::Parenthesized(e) => self.infer_expr_type(e),
            ExprKind::StructInstance { name, .. } => name.clone(),
        }
    }

    fn get_builtin_return_type(&self, name: &str) -> String {
        match name {
            "print" | "push" | "pop" | "time" => "nil".to_string(),
            "input" => "string".to_string(),
            "len" => "int".to_string(),
            "type" | "str" => "string".to_string(),
            "int" | "float" => "number".to_string(),
            "random" => "int".to_string(),
            _ => "function".to_string(),
        }
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
