use sorayunara::lexer::{tokenize, TokenKind};

#[test]
fn test_tokenize_primitives() {
    let source = "let x: Int = 42 let y: Float = 3.14 let s: String = \"aether\" let b: Bool = true";
    let tokens = tokenize(source).expect("Tokenization should succeed");
    
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::IntLit(42))));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::StrLit(ref s) if s == "aether")));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::True)));
}

#[test]
fn test_tokenize_operators() {
    let source = "+ - * / % == != < <= > >= && || ! & -> => :: : ; , .";
    let tokens = tokenize(source).expect("Tokenization should succeed");
    assert!(tokens.len() > 15);
}

#[test]
fn test_tokenize_comments_and_whitespace() {
    let source = "// Single line comment\nlet x = 10 /* Multi\nline\ncomment */ let y = 20";
    let tokens = tokenize(source).expect("Tokenization should succeed");
    assert_eq!(tokens.iter().filter(|t| matches!(t.kind, TokenKind::Let)).count(), 2);
}
