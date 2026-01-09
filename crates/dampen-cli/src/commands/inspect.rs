#![allow(clippy::print_stderr, clippy::print_stdout)]

//! Inspect command - view IR and generated code

use dampen_core::{generate_application, parse, HandlerSignature};
use std::fs;

#[derive(clap::Args)]
pub struct InspectArgs {
    /// Path to the .dampen file to inspect
    #[arg(short, long)]
    file: String,

    /// Show generated Rust code instead of IR
    #[arg(long)]
    codegen: bool,

    /// Output format: human (default) or json
    #[arg(long, default_value = "human")]
    format: String,

    /// Model struct name (used with --codegen)
    #[arg(long, default_value = "Model")]
    model: String,

    /// Message enum name (used with --codegen)
    #[arg(long, default_value = "Message")]
    message: String,

    /// Handler names (for codegen validation, comma-separated)
    #[arg(long, value_delimiter = ',', num_args = 1..)]
    handlers: Vec<String>,
}

pub fn execute(args: &InspectArgs) -> Result<(), String> {
    // Read the file
    let content = fs::read_to_string(&args.file)
        .map_err(|e| format!("Failed to read file '{}': {}", args.file, e))?;

    // Parse the XML
    let document = parse(&content).map_err(|e| format!("Parse error: {}", e))?;

    if args.codegen {
        // Generate code
        let handler_signatures: Vec<HandlerSignature> = args
            .handlers
            .iter()
            .map(|name| {
                HandlerSignature {
                    name: name.clone(),
                    param_type: None, // Could be enhanced to parse type info
                    returns_command: false,
                }
            })
            .collect();

        let output =
            generate_application(&document, &args.model, &args.message, &handler_signatures)
                .map_err(|e| format!("Code generation error: {}", e))?;

        match args.format.as_str() {
            "json" => {
                let json = serde_json::json!({
                    "code": output.code,
                    "warnings": output.warnings,
                });
                let json_str = serde_json::to_string_pretty(&json)
                    .map_err(|e| format!("JSON serialization error: {}", e))?;
                println!("{}", json_str);
            }
            "human" => {
                println!("// Generated Rust code from: {}\n", args.file);
                if !output.warnings.is_empty() {
                    println!("// Warnings:");
                    for warning in &output.warnings {
                        println!("//   - {}", warning);
                    }
                    println!();
                }
                println!("{}", output.code);
            }
            _ => return Err(format!("Unknown format: {}", args.format)),
        }
    } else {
        // Show IR tree
        match args.format.as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&document)
                    .map_err(|e| format!("JSON serialization error: {}", e))?;
                println!("{}", json);
            }
            "human" => {
                print_ir_tree(&document, 0);
            }
            _ => return Err(format!("Unknown format: {}", args.format)),
        }
    }

    Ok(())
}

/// Print IR tree in human-readable format
fn print_ir_tree(doc: &dampen_core::DampenDocument, indent: usize) {
    let prefix = "  ".repeat(indent);

    println!("{}DampenDocument {{", prefix);
    println!(
        "{}  version: v{}.{}",
        prefix, doc.version.major, doc.version.minor
    );
    println!("{}  root:", prefix);
    print_widget_node(&doc.root, indent + 2);
    println!("{}}}", prefix);
}

fn print_widget_node(node: &dampen_core::WidgetNode, indent: usize) {
    let prefix = "  ".repeat(indent);

    println!("{}WidgetNode {{", prefix);
    println!("{}  kind: {:?}", prefix, node.kind);

    if let Some(id) = &node.id {
        println!("{}  id: Some({:?})", prefix, id);
    }

    // Print attributes
    if !node.attributes.is_empty() {
        println!("{}  attributes: {{", prefix);
        for (key, value) in &node.attributes {
            print!("{}    {:?}: ", prefix, key);
            print_attribute_value(value);
            println!();
        }
        println!("{}  }}", prefix);
    }

    // Print events
    if !node.events.is_empty() {
        println!("{}  events: [", prefix);
        for event in &node.events {
            println!(
                "{}    {:?} -> {} (line {}, col {})",
                prefix, event.event, event.handler, event.span.line, event.span.column
            );
        }
        println!("{}  ]", prefix);
    }

    // Print children
    if !node.children.is_empty() {
        println!("{}  children: [", prefix);
        for child in &node.children {
            print_widget_node(child, indent + 2);
        }
        println!("{}  ]", prefix);
    }

    println!(
        "{}  span: line {}, column {}",
        prefix, node.span.line, node.span.column
    );
    println!("{}}}", prefix);
}

fn print_attribute_value(value: &dampen_core::AttributeValue) {
    match value {
        dampen_core::AttributeValue::Static(s) => {
            print!("Static({:?})", s);
        }
        dampen_core::AttributeValue::Binding(expr) => {
            print!("Binding(");
            print_expr(&expr.expr);
            print!(")");
        }
        dampen_core::AttributeValue::Interpolated(parts) => {
            print!("Interpolated([");
            for (i, part) in parts.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                match part {
                    dampen_core::InterpolatedPart::Literal(s) => {
                        print!("Literal({:?})", s);
                    }
                    dampen_core::InterpolatedPart::Binding(expr) => {
                        print!("Binding(");
                        print_expr(&expr.expr);
                        print!(")");
                    }
                }
            }
            print!("])");
        }
    }
}

fn print_expr(expr: &dampen_core::Expr) {
    match expr {
        dampen_core::Expr::FieldAccess(fa) => {
            print!("FieldAccess({})", fa.path.join("."));
        }
        dampen_core::Expr::MethodCall(mc) => {
            print!("MethodCall(");
            print_expr(&mc.receiver);
            print!(".{}(", mc.method);
            for (i, arg) in mc.args.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print_expr(arg);
            }
            print!("))");
        }
        dampen_core::Expr::BinaryOp(bo) => {
            print!("BinaryOp(");
            print_expr(&bo.left);
            print!(" {:?} ", bo.op);
            print_expr(&bo.right);
            print!(")");
        }
        dampen_core::Expr::UnaryOp(uo) => {
            print!("UnaryOp({:?} ", uo.op);
            print_expr(&uo.operand);
            print!(")");
        }
        dampen_core::Expr::Conditional(ce) => {
            print!("Conditional(");
            print_expr(&ce.condition);
            print!(" then ");
            print_expr(&ce.then_branch);
            print!(" else ");
            print_expr(&ce.else_branch);
            print!(")");
        }
        dampen_core::Expr::Literal(lit) => match lit {
            dampen_core::LiteralExpr::String(s) => print!("Literal({:?})", s),
            dampen_core::LiteralExpr::Integer(i) => print!("Literal({})", i),
            dampen_core::LiteralExpr::Float(f) => print!("Literal({})", f),
            dampen_core::LiteralExpr::Bool(b) => print!("Literal({})", b),
        },
    }
}
