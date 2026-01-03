use gravity_core::parse;
use std::fs;

fn main() {
    let xml = fs::read_to_string("/tmp/test_container.xml").unwrap();

    match parse(&xml) {
        Ok(doc) => {
            println!("=== Parsed Document ===");
            println!("Root widget: {:?}", doc.root.kind);
            println!("Root children count: {}", doc.root.children.len());

            // First child should be the outer container with class="card"
            if let Some(outer_container) = doc.root.children.first() {
                println!("\n=== Outer Container (class='card') ===");
                println!("Kind: {:?}", outer_container.kind);
                println!("Classes: {:?}", outer_container.classes);
                println!("Children count: {}", outer_container.children.len());

                // The outer container should have a column as its child
                if let Some(column) = outer_container.children.first() {
                    println!("\n=== Column (spacing='15') ===");
                    println!("Kind: {:?}", column.kind);
                    println!("Children count: {}", column.children.len());

                    for (i, child) in column.children.iter().enumerate() {
                        println!("\n  Child {}: {:?}", i, child.kind);
                        if child.kind == gravity_core::ir::WidgetKind::Text {
                            if let Some(value_attr) = child.attributes.get("value") {
                                println!("    Text value: {:?}", value_attr);
                            }
                        }
                        println!("    Children count: {}", child.children.len());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
