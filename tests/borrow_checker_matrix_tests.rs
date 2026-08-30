use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

fn check_fixture_valid(source: &str) {
    let tokens = tokenize(source).expect("Lexing fixture failed");
    let program = parse(tokens).expect("Parsing fixture failed");
    if let Err(engine) = check_semantics(&program) {
        panic!(
            "Valid fixture failed borrow analysis:\n{}",
            engine.render_all("valid_test.sora", source)
        );
    }
}

fn check_fixture_invalid(source: &str) {
    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(_) => return, // Rejected at lexer
    };
    let program = match parse(tokens) {
        Ok(p) => p,
        Err(_) => return, // Rejected at parser
    };
    assert!(
        check_semantics(&program).is_err(),
        "Invalid fixture passed check_semantics unexpectedly!"
    );
}

#[test]
fn test_matrix_valid_ownership() {
    let src = include_str!("fixtures/borrowchecker/valid/ownership.sora");
    check_fixture_valid(src);
}

#[test]
fn test_matrix_valid_borrowing() {
    let src = include_str!("fixtures/borrowchecker/valid/borrowing.sora");
    check_fixture_valid(src);
}

#[test]
fn test_matrix_valid_mutable_borrow() {
    let src = include_str!("fixtures/borrowchecker/valid/mutable_borrow.sora");
    check_fixture_valid(src);
}

#[test]
fn test_matrix_valid_lifetime() {
    let src = include_str!("fixtures/borrowchecker/valid/lifetime.sora");
    check_fixture_valid(src);
}

#[test]
fn test_matrix_valid_concurrency() {
    let src = include_str!("fixtures/borrowchecker/valid/concurrency.sora");
    check_fixture_valid(src);
}

#[test]
fn test_matrix_invalid_use_after_move() {
    let src = include_str!("fixtures/borrowchecker/invalid/use_after_move.sora");
    check_fixture_invalid(src);
}

#[test]
fn test_matrix_invalid_double_mut_borrow() {
    let src = include_str!("fixtures/borrowchecker/invalid/double_mut_borrow.sora");
    check_fixture_invalid(src);
}

#[test]
fn test_matrix_invalid_dangling_reference() {
    let src = include_str!("fixtures/borrowchecker/invalid/dangling_reference.sora");
    check_fixture_invalid(src);
}

#[test]
fn test_matrix_invalid_data_race() {
    let src = include_str!("fixtures/borrowchecker/invalid/data_race.sora");
    check_fixture_invalid(src);
}

#[test]
fn test_matrix_invalid_lifetime_violation() {
    let src = include_str!("fixtures/borrowchecker/invalid/lifetime_violation.sora");
    check_fixture_invalid(src);
}
