#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use crate::expr::{
    BinaryOp, BinaryOpExpr, BindingExpr, ConditionalExpr, Expr, FieldAccessExpr, LiteralExpr,
    MethodCallExpr, SharedFieldAccessExpr, UnaryOp, UnaryOpExpr,
};
use crate::ir::span::Span;

/// Tokenize and parse a binding expression
pub fn tokenize_binding_expr(
    input: &str,
    start_pos: usize,
    line: u32,
    column: u32,
) -> Result<BindingExpr, String> {
    let mut parser = ExprParser::new(input, start_pos, line, column);
    let expr = parser.parse()?;
    let span = Span::new(start_pos, start_pos + input.len(), line, column);
    Ok(BindingExpr { expr, span })
}

struct ExprParser<'a> {
    input: &'a str,
    pos: usize,
    start_pos: usize,
    line: u32,
    column: u32,
}

impl<'a> ExprParser<'a> {
    fn new(input: &'a str, start_pos: usize, line: u32, column: u32) -> Self {
        Self {
            input,
            pos: 0,
            start_pos,
            line,
            column,
        }
    }

    fn parse(&mut self) -> Result<Expr, String> {
        self.parse_conditional()
    }

    fn parse_conditional(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        if self.peek_keyword("if") {
            self.consume_keyword("if")?;
            self.skip_whitespace();
            let condition = self.parse_or()?;
            self.skip_whitespace();
            self.consume_keyword("then")?;
            self.skip_whitespace();
            let then_branch = self.parse()?;
            self.skip_whitespace();
            self.consume_keyword("else")?;
            self.skip_whitespace();
            let else_branch = self.parse()?;

            return Ok(Expr::Conditional(ConditionalExpr {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            }));
        }

        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;

        loop {
            self.skip_whitespace();
            if self.peek_str("||") {
                self.consume_str("||")?;
                self.skip_whitespace();
                let right = self.parse_and()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Or,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        loop {
            self.skip_whitespace();
            if self.peek_str("&&") {
                self.consume_str("&&")?;
                self.skip_whitespace();
                let right = self.parse_comparison()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::And,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;

        loop {
            self.skip_whitespace();

            if self.peek_str("==") {
                self.consume_str("==")?;
                self.skip_whitespace();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Eq,
                    right: Box::new(right),
                });
            } else if self.peek_str("!=") {
                self.consume_str("!=")?;
                self.skip_whitespace();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Ne,
                    right: Box::new(right),
                });
            } else if self.peek_str("<=") {
                self.consume_str("<=")?;
                self.skip_whitespace();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Le,
                    right: Box::new(right),
                });
            } else if self.peek_str(">=") {
                self.consume_str(">=")?;
                self.skip_whitespace();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Ge,
                    right: Box::new(right),
                });
            } else if self.peek_str("<") {
                self.consume_str("<")?;
                self.skip_whitespace();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Lt,
                    right: Box::new(right),
                });
            } else if self.peek_str(">") {
                self.consume_str(">")?;
                self.skip_whitespace();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Gt,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        loop {
            self.skip_whitespace();

            if self.peek_str("+") {
                self.consume_str("+")?;
                self.skip_whitespace();
                let right = self.parse_multiplicative()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Add,
                    right: Box::new(right),
                });
            } else if self.peek_str("-") {
                self.consume_str("-")?;
                self.skip_whitespace();
                let right = self.parse_multiplicative()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Sub,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            self.skip_whitespace();

            if self.peek_str("*") {
                self.consume_str("*")?;
                self.skip_whitespace();
                let right = self.parse_unary()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Mul,
                    right: Box::new(right),
                });
            } else if self.peek_str("/") {
                self.consume_str("/")?;
                self.skip_whitespace();
                let right = self.parse_unary()?;
                left = Expr::BinaryOp(BinaryOpExpr {
                    left: Box::new(left),
                    op: BinaryOp::Div,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();

        if self.peek_str("!") {
            self.consume_str("!")?;
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp(UnaryOpExpr {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            }));
        } else if self.peek_str("-") {
            self.consume_str("-")?;
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp(UnaryOpExpr {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            }));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();

        // Literal string
        if self.peek_str("\"") || self.peek_str("'") {
            return self.parse_string_literal();
        }

        // Literal number
        if self.peek_digit() || self.peek_str("-") {
            return self.parse_number_literal();
        }

        // Literal boolean
        if self.peek_keyword("true") {
            self.consume_keyword("true")?;
            return Ok(Expr::Literal(LiteralExpr::Bool(true)));
        }
        if self.peek_keyword("false") {
            self.consume_keyword("false")?;
            return Ok(Expr::Literal(LiteralExpr::Bool(false)));
        }

        // Field access or method call
        if self.peek_alpha() || self.peek_str("_") {
            return self.parse_field_or_method();
        }

        // Parenthesized expression
        if self.peek_str("(") {
            self.consume_str("(")?;
            let expr = self.parse()?;
            self.skip_whitespace();
            self.consume_str(")")?;
            return Ok(expr);
        }

        Err(format!("Unexpected character at position {}", self.pos))
    }

    fn parse_field_or_method(&mut self) -> Result<Expr, String> {
        let mut path = Vec::new();

        // Parse identifier
        let ident = self.parse_identifier()?;

        // Check if this is a shared state access: `shared.field`
        let is_shared = ident == "shared";

        if is_shared {
            // Must have a dot and at least one field after "shared"
            self.skip_whitespace();
            if !self.peek_str(".") {
                // Just "shared" by itself - treat as regular field access (for backward compat)
                path.push(ident);
            } else {
                // Consume the dot after "shared"
                self.consume_str(".")?;

                // Parse the shared field path
                let mut shared_path = Vec::new();
                let first_field = self.parse_identifier()?;
                shared_path.push(first_field);

                // Check for nested shared field access
                loop {
                    self.skip_whitespace();
                    if self.peek_str(".") {
                        self.consume_str(".")?;
                        let next_ident = self.parse_identifier()?;
                        shared_path.push(next_ident);
                    } else {
                        break;
                    }
                }

                // Check for method call on shared field
                self.skip_whitespace();
                if self.peek_str("(") {
                    self.consume_str("(")?;
                    let mut args = Vec::new();

                    // Parse arguments
                    loop {
                        self.skip_whitespace();
                        if self.peek_str(")") {
                            break;
                        }

                        let arg = self.parse()?;
                        args.push(arg);

                        self.skip_whitespace();
                        if self.peek_str(",") {
                            self.consume_str(",")?;
                        } else {
                            break;
                        }
                    }

                    self.consume_str(")")?;

                    // Method call on the shared path
                    let method = shared_path.pop().ok_or("Empty path for method call")?;
                    let receiver = if shared_path.is_empty() {
                        // Just `shared.method()` - receiver is the entire shared context
                        Expr::SharedFieldAccess(SharedFieldAccessExpr { path: vec![] })
                    } else {
                        Expr::SharedFieldAccess(SharedFieldAccessExpr { path: shared_path })
                    };

                    return Ok(Expr::MethodCall(MethodCallExpr {
                        receiver: Box::new(receiver),
                        method,
                        args,
                    }));
                }

                // Just shared field access
                return Ok(Expr::SharedFieldAccess(SharedFieldAccessExpr {
                    path: shared_path,
                }));
            }
        } else {
            path.push(ident);
        }

        // Check for nested field access
        loop {
            self.skip_whitespace();
            if self.peek_str(".") {
                self.consume_str(".")?;
                let next_ident = self.parse_identifier()?;
                path.push(next_ident);
            } else {
                break;
            }
        }

        // Check for method call
        self.skip_whitespace();
        if self.peek_str("(") {
            self.consume_str("(")?;
            let mut args = Vec::new();

            // Parse arguments
            loop {
                self.skip_whitespace();
                if self.peek_str(")") {
                    break;
                }

                let arg = self.parse()?;
                args.push(arg);

                self.skip_whitespace();
                if self.peek_str(",") {
                    self.consume_str(",")?;
                } else {
                    break;
                }
            }

            self.consume_str(")")?;

            // Method call on the path
            let method = path.pop().ok_or("Empty path for method call")?;
            let receiver = if path.is_empty() {
                Expr::Literal(LiteralExpr::String("self".to_string()))
            } else {
                Expr::FieldAccess(FieldAccessExpr { path })
            };

            return Ok(Expr::MethodCall(MethodCallExpr {
                receiver: Box::new(receiver),
                method,
                args,
            }));
        }

        // Just field access
        Ok(Expr::FieldAccess(FieldAccessExpr { path }))
    }

    fn parse_string_literal(&mut self) -> Result<Expr, String> {
        let quote = if self.peek_str("\"") { "\"" } else { "'" };
        self.consume_str(quote)?;

        let mut value = String::new();
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.to_string() == quote {
                self.pos += 1;
                break;
            }
            if c == '\\' {
                // Handle escape sequences
                self.pos += 1;
                if self.pos < self.input.len() {
                    let next = self.input[self.pos..].chars().next().unwrap();
                    value.push(next);
                    self.pos += next.len_utf8();
                }
            } else {
                value.push(c);
                self.pos += c.len_utf8();
            }
        }

        Ok(Expr::Literal(LiteralExpr::String(value)))
    }

    fn parse_number_literal(&mut self) -> Result<Expr, String> {
        let start = self.pos;
        let mut is_float = false;

        if self.peek_str("-") {
            self.pos += 1;
        }

        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.is_ascii_digit() {
                self.pos += 1;
            } else if c == '.' && !is_float {
                is_float = true;
                self.pos += 1;
            } else {
                break;
            }
        }

        let num_str = &self.input[start..self.pos];

        if is_float {
            let value: f64 = num_str
                .parse()
                .map_err(|e| format!("Invalid float: {}", e))?;
            Ok(Expr::Literal(LiteralExpr::Float(value)))
        } else {
            let value: i64 = num_str
                .parse()
                .map_err(|e| format!("Invalid integer: {}", e))?;
            Ok(Expr::Literal(LiteralExpr::Integer(value)))
        }
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        let start = self.pos;

        if self.pos >= self.input.len() {
            return Err("Expected identifier".to_string());
        }

        let first = self.input[self.pos..].chars().next().unwrap();
        if !first.is_alphabetic() && first != '_' {
            return Err(format!("Expected identifier, got '{}'", first));
        }

        self.pos += first.len_utf8();

        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.is_alphanumeric() || c == '_' {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }

        Ok(self.input[start..self.pos].to_string())
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            if let Some(c) = self.input[self.pos..].chars().next() {
                if c.is_whitespace() {
                    self.pos += c.len_utf8();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn peek_str(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn peek_keyword(&self, keyword: &str) -> bool {
        let remaining = &self.input[self.pos..];
        if !remaining.starts_with(keyword) {
            return false;
        }
        // Check that it's followed by whitespace or non-identifier
        if let Some(next_char) = remaining[keyword.len()..].chars().next() {
            !next_char.is_alphanumeric() && next_char != '_'
        } else {
            true
        }
    }

    fn peek_digit(&self) -> bool {
        self.input[self.pos..]
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
    }

    fn peek_alpha(&self) -> bool {
        self.input[self.pos..]
            .chars()
            .next()
            .map(|c| c.is_alphabetic())
            .unwrap_or(false)
    }

    fn consume_str(&mut self, s: &str) -> Result<(), String> {
        if self.peek_str(s) {
            self.pos += s.len();
            Ok(())
        } else {
            Err(format!("Expected '{}'", s))
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> Result<(), String> {
        if self.peek_keyword(keyword) {
            self.pos += keyword.len();
            Ok(())
        } else {
            Err(format!("Expected keyword '{}'", keyword))
        }
    }
}
