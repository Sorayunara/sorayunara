use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;

#[test]
fn test_fuzz_random_malformed_inputs() {
    let inputs = vec![
        "fn {{{",
        "let mut = == => ->",
        "struct { id: }",
        "match { => => }",
        "import \"\"\"\" . . .",
        "spawn ()()()",
        "123.456.789",
        "// unterminated comment /*",
    ];

    for input in inputs {
        // Must gracefully return Err instead of crashing or panicking
        if let Ok(tokens) = tokenize(input) {
            let _ = parse(tokens);
        }
    }
}
