use sorayunara::docgen::{extract_doc_comments, generate_html_docs};
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use std::fs;
use std::path::Path;

#[test]
fn test_docgen_extract_doc_comments() {
    let source = r#"
/// Calculates the total price.
///
/// # Arguments
/// - `items`: list of products
fn calculate_total(items: Int) -> Int {
    return items * 10
}
"#;

    let docs = extract_doc_comments(source);
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].0, "calculate_total");
    assert!(docs[0].1.contains("Calculates the total price."));
    assert!(docs[0].1.contains("# Arguments"));
    assert!(docs[0].1.contains("- `items`: list of products"));
}

#[test]
fn test_docgen_html_hierarchy_generation() {
    let source = r#"
/// Product item representing store inventory.
struct Item {
    id: Int,
    price: Int
}

/// Calculates the total price of all items.
///
/// # Arguments
/// - `items`: list of products
fn calculate_total(items: Int) -> Int {
    return items * 10
}
"#;

    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    let output_dir = Path::new("target/test_docs");
    if output_dir.exists() {
        let _ = fs::remove_dir_all(output_dir);
    }

    generate_html_docs(&program, source, "Store Module", output_dir).unwrap();

    assert!(output_dir.join("index.html").exists());
    assert!(output_dir.join("modules").join("index.html").exists());
    assert!(output_dir.join("traits").join("index.html").exists());
    assert!(output_dir.join("functions").join("calculate_total.html").exists());
    assert!(output_dir.join("structs").join("Item.html").exists());

    let fn_html = fs::read_to_string(output_dir.join("functions").join("calculate_total.html")).unwrap();
    assert!(fn_html.contains("calculate_total"));
    assert!(fn_html.contains("Calculates the total price"));
    assert!(fn_html.contains("# Arguments"));
}
