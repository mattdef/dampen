use gravity_core::parse;

fn main() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<gravity>
    <container class="card">
        <column spacing="15">
            <text value="2. Layout Constraints &amp; Sizing" size="20" weight="bold" color="#2c3e50" />
            
            <!-- Fixed width -->
            <container background="#3498db" padding="10" border_radius="4" width="200">
                <text value="Fixed width: 200px" size="12" color="#ffffff" />
            </container>
            
            <!-- Fill width -->
            <container background="#2ecc71" padding="10" border_radius="4" width="fill">
                <text value="Fill width: Expands to available space" size="12" color="#ffffff" />
            </container>
        </column>
    </container>
</gravity>"#;

    match parse(xml) {
        Ok(doc) => {
            println!("Root widget: {:?}", doc.root.kind);
            println!("Root children count: {}", doc.root.children.len());
            
            if let Some(first_child) = doc.root.children.first() {
                println!("First child kind: {:?}", first_child.kind);
                println!("First child classes: {:?}", first_child.classes);
                println!("First child children count: {}", first_child.children.len());
                
                if let Some(column) = first_child.children.first() {
                    println!("Column kind: {:?}", column.kind);
                    println!("Column children count: {}", column.children.len());
                    
                    for (i, child) in column.children.iter().enumerate() {
                        println!("  Child {}: {:?}", i, child.kind);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
