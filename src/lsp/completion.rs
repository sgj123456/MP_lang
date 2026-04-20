use crate::lexer::{TokenKind, tokenize};
use crate::parser::{StmtKind, parse};
use tower_lsp::lsp_types::*;

#[derive(Debug)]
pub struct MpCompleter {
    keywords: Vec<&'static str>,
    builtin_functions: Vec<&'static str>,
    #[allow(dead_code)]
    builtin_types: Vec<&'static str>,
}

impl Default for MpCompleter {
    fn default() -> Self {
        Self::new()
    }
}

impl MpCompleter {
    pub fn new() -> Self {
        Self {
            keywords: vec![
                "let", "fn", "if", "else", "while", "return", "break", "continue", "true", "false",
                "nil",
            ],
            builtin_functions: vec![
                "print", "input", "len", "type", "str", "int", "float", "random", "push", "pop", "time",
            ],
            builtin_types: vec![
                "Number", "String", "Boolean", "Array", "Object", "Function", "Nil",
            ],
        }
    }

    pub fn complete(&self, content: &str, position: Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        let lines: Vec<&str> = content.lines().collect();
        if position.line as usize >= lines.len() {
            return items;
        }

        let current_line = lines[position.line as usize];
        let char_index = position.character as usize;

        if char_index == 0 || current_line.is_empty() {
            items.extend(self.get_keyword_completions());
            items.extend(self.get_builtin_function_completions());
            return items;
        }

        let before_cursor = if char_index <= current_line.len() {
            &current_line[..char_index]
        } else {
            current_line
        };

        if let Ok(tokens) = tokenize(content) {
            let mut in_function_call = false;
            let mut last_identifier = None;

            for token in tokens.iter().rev() {
                match &token.kind {
                    TokenKind::Identifier(name) => {
                        last_identifier = Some(name.clone());
                        break;
                    }
                    TokenKind::LeftParen => {
                        in_function_call = true;
                        break;
                    }
                    TokenKind::Newline | TokenKind::Semicolon => break,
                    _ => {}
                }
            }

            if in_function_call && let Some(func_name) = last_identifier {
                items.extend(self.get_function_argument_completions(&func_name));
            }
        }

        let word_start = before_cursor
            .char_indices()
            .rev()
            .find(|(_, c)| !c.is_alphanumeric() && *c != '_' && *c != ':')
            .map(|(i, _)| i + 1)
            .unwrap_or(0);

        let current_word = &before_cursor[word_start..];

        if current_word.is_empty() || current_word.ends_with(':') || current_word.ends_with('.') {
            items.extend(self.get_keyword_completions());
            items.extend(self.get_builtin_function_completions());

            if let Ok(_tokens) = tokenize(content) {
                items.extend(self.get_variable_completions(content, position));
            }
        } else {
            let current_word_lower = current_word.to_lowercase();

            for keyword in &self.keywords {
                if keyword.to_lowercase().starts_with(&current_word_lower) {
                    items.push(CompletionItem {
                        label: keyword.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        detail: Some("Keyword".to_string()),
                        ..Default::default()
                    });
                }
            }

            for func in &self.builtin_functions {
                if func.to_lowercase().starts_with(&current_word_lower) {
                    items.push(CompletionItem {
                        label: func.to_string(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some("Built-in function".to_string()),
                        documentation: Some(Documentation::String(
                            self.get_function_documentation(func),
                        )),
                        ..Default::default()
                    });
                }
            }

            if let Ok(_tokens) = tokenize(content) {
                for item in self.get_variable_completions(content, position) {
                    if item.label.to_lowercase().starts_with(&current_word_lower) {
                        items.push(item);
                    }
                }
            }
        }

        items
    }

    fn get_keyword_completions(&self) -> Vec<CompletionItem> {
        self.keywords
            .iter()
            .map(|kw| {
                let detail = match *kw {
                    "let" => "Variable declaration",
                    "fn" => "Function definition",
                    "if" => "Conditional statement",
                    "else" => "Else branch",
                    "while" => "Loop statement",
                    "return" => "Return from function",
                    "break" => "Break from loop",
                    "continue" => "Continue to next iteration",
                    "true" => "Boolean true",
                    "false" => "Boolean false",
                    "nil" => "Null value",
                    _ => "Keyword",
                };

                CompletionItem {
                    label: kw.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some(detail.to_string()),
                    ..Default::default()
                }
            })
            .collect()
    }

    fn get_builtin_function_completions(&self) -> Vec<CompletionItem> {
        self.builtin_functions
            .iter()
            .map(|func| CompletionItem {
                label: func.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Built-in function".to_string()),
                documentation: Some(Documentation::String(self.get_function_documentation(func))),
                ..Default::default()
            })
            .collect()
    }

    fn get_variable_completions(&self, content: &str, position: Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let mut variables: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        if let Ok(tokens) = tokenize(content) {
            if let Ok(ast) = parse(tokens.clone()) {
                for stmt in &ast {
                    match &stmt.kind {
                        StmtKind::Let { name, value } => {
                            let var_type = self.infer_type(value);
                            variables.insert(name.clone(), var_type);
                        }
                        StmtKind::Function { name, params, .. } => {
                            let params_str = params.join(", ");
                            variables.insert(name.clone(), format!("fn({})", params_str));
                        }
                        _ => {}
                    }
                }
            }

            let mut iter = tokens.iter().peekable();
            while let Some(token) = iter.next() {
                if let TokenKind::Identifier(name) = &token.kind
                    && token.span.line <= position.line as usize
                    && let Some(next_token) = iter.peek()
                        && matches!(next_token.kind, TokenKind::Assign)
                        && !variables.contains_key(name) {
                            variables.insert(name.clone(), "Unknown".to_string());
                        }
            }
        }

        for (var, var_type) in variables {
            let kind = if var_type.starts_with("fn(") {
                Some(CompletionItemKind::FUNCTION)
            } else {
                Some(CompletionItemKind::VARIABLE)
            };

            items.push(CompletionItem {
                label: var,
                kind,
                detail: Some(var_type),
                ..Default::default()
            });
        }

        items
    }

    fn infer_type(&self, expr: &crate::parser::Expr) -> String {
        use crate::parser::ExprKind;
        match &expr.kind {
            ExprKind::Number(_) => "Number".to_string(),
            ExprKind::String(_) => "String".to_string(),
            ExprKind::Boolean(_) => "Boolean".to_string(),
            ExprKind::Array(items) => {
                if let Some(first) = items.first() {
                    let elem_type = self.infer_type(first);
                    format!("Array<{}>", elem_type)
                } else {
                    "Array".to_string()
                }
            }
            ExprKind::Object(fields) => {
                if fields.is_empty() {
                    "Object".to_string()
                } else {
                    let field_types: Vec<String> = fields
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, self.infer_type(v)))
                        .collect();
                    format!("Object {{ {} }}", field_types.join(", "))
                }
            }
            ExprKind::FunctionCall { name, args } => self.get_function_return_type(name, args),
            ExprKind::Variable(_) => "Unknown".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn get_function_return_type(&self, name: &str, args: &[crate::parser::Expr]) -> String {
        match name {
            "len" => "Number".to_string(),
            "type" => "String".to_string(),
            "str" => "String".to_string(),
            "int" => "Number".to_string(),
            "float" => "Number".to_string(),
            "input" => "String".to_string(),
            "random" => "Number".to_string(),
            "time" => "Number".to_string(),
            "push" => {
                if let Some(first) = args.first() {
                    self.infer_type(first)
                } else {
                    "Unknown".to_string()
                }
            }
            "pop" => {
                if let Some(first) = args.first() {
                    self.infer_type(first)
                } else {
                    "Unknown".to_string()
                }
            }
            "print" => "Nil".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn get_function_argument_completions(&self, func_name: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        match func_name {
            "print" => {
                items.push(CompletionItem {
                    label: "value".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Any value to print".to_string()),
                    ..Default::default()
                });
            }
            "len" => {
                items.push(CompletionItem {
                    label: "collection".to_string(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some("String, array, or object".to_string()),
                    ..Default::default()
                });
            }
            "type" => {
                items.push(CompletionItem {
                    label: "value".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Any value".to_string()),
                    ..Default::default()
                });
            }
            "str" | "int" | "float" => {
                items.push(CompletionItem {
                    label: "value".to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("Value to convert".to_string()),
                    ..Default::default()
                });
            }
            _ => {}
        }

        items
    }

    fn get_function_documentation(&self, func_name: &str) -> String {
        match func_name {
            "print" => "print(expr) - Print the value of expr to the console".to_string(),
            "input" => "input() - Read a string from the console".to_string(),
            "len" => "len(str) - Return the length of str (string, array, or object)".to_string(),
            "type" => "type(expr) - Return the type of expr as a string".to_string(),
            "str" => "str(num) - Convert num to a string".to_string(),
            "int" => "int(str) - Convert str to an integer".to_string(),
            "float" => "float(str) - Convert str to a float".to_string(),
            "random" => {
                "random() | random(max) | random(min, max) - Generate random number".to_string()
            }
            "push" => "push(array, item) - Add item to array".to_string(),
            "pop" => "pop(array) - Remove and return last item from array".to_string(),
            "time" => "time() - Get current Unix timestamp in seconds".to_string(),
            _ => "Built-in function".to_string(),
        }
    }
}
