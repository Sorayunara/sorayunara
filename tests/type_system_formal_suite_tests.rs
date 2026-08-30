use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

fn check_valid(src: &str) {
    let tokens = tokenize(src).expect("Lexing failed");
    let program = parse(tokens).expect("Parsing failed");
    if let Err(engine) = check_semantics(&program) {
        panic!("Semantic check failed:\n{}", engine.render_all("test.sora", src));
    }
}

fn check_invalid(src: &str) {
    let tokens = tokenize(src).expect("Lexing failed");
    let program = parse(tokens).expect("Parsing failed");
    assert!(
        check_semantics(&program).is_err(),
        "Expected typecheck error, but succeeded"
    );
}

#[test]
fn test_type_primitives_and_tuples() {
    let src = r#"
        fn main() {
            let a: Int = 100
            let b: Float = 3.14
            let c: Bool = true
            let d: String = "Sora"
            let t: (Int, Bool) = (10, true)
        }
    "#;
    check_valid(src);
}

#[test]
fn test_type_arrays_and_indexing() {
    let src = r#"
        fn main() {
            let arr: [Int] = [1, 2, 3, 4]
            let first: Int = arr[0]
        }
    "#;
    check_valid(src);
}

#[test]
fn test_type_algebraic_data_types_and_exhaustive_match() {
    let src = r#"
        enum Status {
            Success(Int)
            Failure(String)
            Pending
        }

        fn evaluate(s: Status) -> Int {
            let res = match s {
                Success(v) => v
                Failure(e) => 0
                Pending => -1
            }
            return res
        }
    "#;
    check_valid(src);
}

#[test]
fn test_type_generics_and_monomorphization() {
    let src = r#"
        fn identity<T>(val: T) -> T {
            return val
        }

        fn main() {
            let num: Int = identity(42)
            let text: String = identity("test")
        }
    "#;
    check_valid(src);
}

#[test]
fn test_type_options_and_results() {
    let src = r#"
        fn get_val(flag: Bool) -> Option<Int> {
            if flag {
                return Some(42)
            } else {
                return None
            }
        }

        fn check_result(flag: Bool) -> Result<Int, String> {
            if flag {
                return Ok(100)
            } else {
                return Err("Failed")
            }
        }
    "#;
    check_valid(src);
}

#[test]
fn test_type_borrowing_and_mutability_safety() {
    let valid_src = r#"
        fn pass_ref(val: &Int) -> &Int {
            return val
        }

        fn main() {
            let x: Int = 42
            let r: &Int = &x
            let y: &Int = pass_ref(r)
        }
    "#;
    check_valid(valid_src);
}

#[test]
fn test_type_mismatch_rejection() {
    let invalid_src = r#"
        fn main() {
            let x: Int = "not an integer"
        }
    "#;
    check_invalid(invalid_src);
}
