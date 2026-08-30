use sorayunara::diagnostic::{Diagnostic, DiagnosticEngine, ErrorCode, Span};

#[test]
fn test_rich_diagnostics_rustc_style_formatting() {
    let source = "let count: Int = \"hello\"\nlet y = 20";
    let span = Span::new(17, 24, 1, 18);

    let diag = Diagnostic::error("mismatched types", span)
        .with_code(ErrorCode::E0308)
        .with_label(span, "expected `Int`, found `String`", true)
        .with_note("expected integer due to type annotation")
        .with_suggestion("convert the value to `Int` via `.parse_int()`");

    let mut engine = DiagnosticEngine::new();
    engine.emit(diag);

    let rendered = engine.render_all("main.sora", source);

    assert!(rendered.contains("error[E0308]: mismatched types"));
    assert!(rendered.contains("--> main.sora:1:18"));
    assert!(rendered.contains("let count: Int = \"hello\""));
    assert!(rendered.contains("^^^^^^^ expected `Int`, found `String`"));
    assert!(rendered.contains("= note: expected integer due to type annotation"));
    assert!(rendered.contains("= help: convert the value to `Int` via `.parse_int()`"));
}
