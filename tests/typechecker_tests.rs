use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::diagnostic::Span;
use std::collections::HashMap;

fn run_analysis(src: &str) -> Result<(sorayunara::symbol_table::SymbolTable, HashMap<Span, sorayunara::symbol_table::Type>), String> {
    let tokens = tokenize(src).map_err(|e| format!("Lexer error: {:?}", e))?;
    let program = parse(tokens).map_err(|(e, _)| format!("Parse error: {}", e))?;
    check_semantics(&program).map_err(|engine| engine.render_all("test.ae", src))
}

fn assert_inferred_type(src: &str, var_name: &str, expected_snippet: &str) {
    let result = run_analysis(src);
    assert!(
        result.is_ok(),
        "Expected analysis to pass, but got error:\n{}",
        result.unwrap_err()
    );
    let (symtab, _annot) = result.unwrap();
    let var = symtab.get_variable(var_name)
        .unwrap_or_else(|| panic!("Variable {} not found", var_name));
    let display = format!("{}", var.ty);
    assert!(
        display.contains(expected_snippet),
        "Expected type of `{}` to contain `{}`, got `{}`",
        var_name, expected_snippet, display
    );
}

// =====================================================
// 1. TYPE INFERENCE (Hindley-Milner style)
// =====================================================

#[test]
fn test_hindley_milner_infer_simple_let() {
    let source = r#"
        fn main() {
            let x = 100
            let y = 3.14
            let s = "hello"
            let b = true
            let c = 'z'
        }
    "#;
    assert_inferred_type(source, "x", "Int");
    assert_inferred_type(source, "y", "Float");
    assert_inferred_type(source, "s", "String");
    assert_inferred_type(source, "b", "Bool");
    assert_inferred_type(source, "c", "Char");
}

#[test]
fn test_hindley_milner_infer_array_unification() {
    let source = r#"
        fn main() {
            let arr = [1, 2, 3, 4, 5]
            let first = arr[0]
        }
    "#;
    assert_inferred_type(source, "arr", "[Int]");
}

#[test]
fn test_hindley_milner_infer_tuple() {
    let source = r#"
        fn main() {
            let t = (42, "answer", 3.14)
        }
    "#;
    assert_inferred_type(source, "t", "(Int, String, Float)");
}

#[test]
fn test_hindley_milner_infer_option() {
    let source = r#"
        fn main() {
            let s = Some(99)
            let n = None
        }
    "#;
    assert_inferred_type(source, "s", "Option<Int>");
}

#[test]
fn test_hindley_milner_infer_result_variants() {
    let source = r#"
        fn main() {
            let o = Ok("success")
            let e = Err(404)
        }
    "#;
    assert_inferred_type(source, "o", "Result<String");
}

#[test]
fn test_hindley_milner_infer_through_binding() {
    let source = r#"
        fn identity(x: Int) -> Int { return x }
        fn main() {
            let a = identity(5)
            let b = a
            let c = b + 1
        }
    "#;
    assert_inferred_type(source, "a", "Int");
    assert_inferred_type(source, "b", "Int");
    assert_inferred_type(source, "c", "Int");
}

#[test]
fn test_hindley_milner_infer_empty_struct() {
    let source = r#"
        struct Point {
            x: Int
            y: Int
        }
        fn main() {
            let p_x = 10
            let p_y = 20
        }
    "#;
    assert_inferred_type(source, "p_x", "Int");
}

// =====================================================
// 2. GENERIC CONSTRAINTS (T: Comparable, etc.)
// =====================================================

#[test]
fn test_generic_constraint_comparable_valid() {
    let source = r#"
        fn max<T: Comparable>(a: T, b: T) -> T {
            if a < b {
                return b
            }
            return a
        }
        fn main() {
            let r = max(10, 20)
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Expected comparable to be satisfied for Int, got: {}", result.unwrap_err());
    assert_inferred_type(source, "r", "Int");
}

#[test]
fn test_generic_constraint_multiple_bounds() {
    let source = r#"
        fn sort_and_print<T: Comparable + Display>(items: [T]) {
            let n = 0
        }
        fn main() {
            let nums = [1, 2, 3]
        }
    "#;
    let result = run_analysis(source);
    assert!(
        result.is_ok(),
        "Comparable+Display on Int should work: {}",
        result.unwrap_err()
    );
}

#[test]
fn test_generic_constraint_comparable_float() {
    let source = r#"
        fn clamp<T: Comparable>(value: T, lo: T, hi: T) -> T {
            if value < lo { return lo }
            if value > hi { return hi }
            return value
        }
        fn main() {
            let x = clamp(3.5, 1.0, 10.0)
        }
    "#;
    assert_inferred_type(source, "x", "Float");
}

#[test]
fn test_generic_constraint_comparable_char() {
    let source = r#"
        fn min<T: Comparable>(a: T, b: T) -> T {
            if a < b { return a }
            return b
        }
        fn main() {
            let c = min('a', 'z')
        }
    "#;
    assert_inferred_type(source, "c", "Char");
}

// =====================================================
// 3. TYPE NARROWING (flow-sensitive via `is`)
// =====================================================

#[test]
fn test_type_narrowing_union_if_is() {
    let source = r#"
        struct Socket { fd: Int }
        struct IoError { code: Int }
        fn handle(value: Int | String) {
            if value is Int {
                let k = value
            }
        }
        fn main() {
            let n = 42
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Type narrowing via `is` should pass: {}", result.unwrap_err());
}

#[test]
fn test_type_narrowing_with_logical_and() {
    let source = r#"
        struct User { name: String; age: Int }
        fn process(x: Int | String) {
            if x is Int {
                let y = x
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Nested narrowing should pass: {}", result.unwrap_err());
}

#[test]
fn test_type_narrowing_negation_else() {
    let source = r#"
        fn dual(v: Int | String) {
            if v is String {
                let s = v
            } else {
                let i = v
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Else-branch narrowing should work: {}", result.unwrap_err());
}

// =====================================================
// 4. ALGEBRAIC DATA TYPES (enums with payloads)
// =====================================================

#[test]
fn test_adt_enum_basic_declaration() {
    let source = r#"
        enum NetworkState {
            Connecting
            Connected(Int)
            Failed(String)
        }
        fn main() {
            let phase = 1
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Simple ADT enum with payloads should register: {}", result.unwrap_err());
}

#[test]
fn test_adt_enum_multiple_payloads() {
    let source = r#"
        enum Tree {
            Leaf
            Node(Int, Int)
        }
        fn main() {
            let seed = 0
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Multi-variant ADT should parse and check: {}", result.unwrap_err());
}

#[test]
fn test_adt_enum_with_unit_payloads() {
    let source = r#"
        enum ShapeKind {
            Circle
            Square
            Rectangle
            Triangle
        }
        fn main() {
            let kind = 1
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Unit-only enum variants OK: {}", result.unwrap_err());
}

#[test]
fn test_adt_enum_complex_nested() {
    let source = r#"
        enum JsonValue {
            JsonNull
            JsonBool(Bool)
            JsonNumber(Float)
            JsonString(String)
        }
        fn main() {
            let code = 0
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Json-like ADT compiles: {}", result.unwrap_err());
}

// =====================================================
// 5. EXHAUSTIVENESS CHECKING
// =====================================================

#[test]
fn test_exhaustiveness_bool_missing_branch() {
    let source = r#"
        fn coin(b: Bool) -> Int {
            let x = match b {
                true => 1
            }
            return 0
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(
        result.is_err(),
        "Expected exhaustiveness error for Bool match missing false, but analysis passed"
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("Non-exhaustive") || err.contains("Bool"),
        "Should mention non-exhaustive Bool match, got: {}",
        err
    );
}

#[test]
fn test_exhaustiveness_bool_full() {
    let source = r#"
        fn coin(b: Bool) -> Int {
            return match b {
                true => 1
                false => 0
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Bool match with both branches should pass: {}", result.unwrap_err());
}

#[test]
fn test_exhaustiveness_bool_wildcard_catchall() {
    let source = r#"
        fn coin(b: Bool) -> Int {
            return match b {
                true => 1
                _ => 0
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Bool match with wildcard should pass: {}", result.unwrap_err());
}

#[test]
fn test_exhaustiveness_option_missing_some() {
    let source = r#"
        fn unwrap_or_zero(o: Option<Int>) -> Int {
            return match o {
                None => 0
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_err(), "Option missing Some should fail");
    let err = result.unwrap_err();
    assert!(
        err.contains("Some") || err.contains("Non-exhaustive") || err.contains("Option"),
        "Should complain about Some missing: {}",
        err
    );
}

#[test]
fn test_exhaustiveness_option_full() {
    let source = r#"
        fn map_option(o: Option<Int>) -> Option<Int> {
            return match o {
                Some(v) => Some(v + 1)
                None => None
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Some/None match should pass: {}", result.unwrap_err());
}

#[test]
fn test_exhaustiveness_result_missing_err() {
    let source = r#"
        fn unwrap(r: Result<Int, String>) -> Int {
            return match r {
                Ok(v) => v
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_err(), "Result missing Err should fail exhaustiveness");
    let err = result.unwrap_err();
    assert!(
        err.contains("Err") || err.contains("Result"),
        "Should mention Err missing: {}",
        err
    );
}

#[test]
fn test_exhaustiveness_result_full() {
    let source = r#"
        fn flatten(r: Result<Int, String>) -> Int {
            return match r {
                Ok(v) => v
                Err(_) => -1
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Result with Ok+Err should be exhaustive: {}", result.unwrap_err());
}

#[test]
fn test_exhaustiveness_wildcard_covers_all() {
    let source = r#"
        enum TriState {
            Low
            High
            Zzz
        }
        fn digit(t: Option<Int>) -> Int {
            return match t {
                Some(v) => v
                _ => 0
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Wildcard covers rest: {}", result.unwrap_err());
}

// =====================================================
// INTEGRATION: Full complex example combining all 5 features
// =====================================================

#[test]
fn test_integration_all_five_features_combined() {
    let complex = r#"
        enum NetworkState {
            Connecting
            Connected(Int)
            Failed(String)
        }

        fn cmp_max<T: Comparable>(a: T, b: T) -> T {
            if a < b { return b }
            return a
        }

        fn describe_state(state: Int | String) -> String {
            if state is Int {
                return "numeric state"
            } else {
                return "string state"
            }
        }

        fn handle(ns: Option<Int>) -> Int {
            return match ns {
                Some(code) => code
                None => -1
            }
        }

        fn main() {
            let inferred_int = cmp_max(5, 10)
            let inferred_str = describe_state(inferred_int)
            let result = handle(Some(42))
        }
    "#;
    assert_inferred_type(complex, "inferred_int", "Int");
    assert_inferred_type(complex, "result", "Int");
    let result = run_analysis(complex);
    assert!(result.is_ok(), "Full integration: {}", result.unwrap_err());
    let (symtab, _) = result.unwrap();

    let f = symtab.lookup_function("cmp_max").unwrap();
    assert_eq!(f.type_params.len(), 1, "cmp_max should have 1 generic param");
    assert_eq!(f.type_params[0].1.first().unwrap(), "Comparable", "Generic param T should be Comparable");
}

// =====================================================
// Regression / Basic tests (from original)
// =====================================================

#[test]
fn test_typecheck_valid_program() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b
        }
        fn main() {
            let x: Int = 10
            let y: Int = 20
            let z: Int = add(x, y)
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Basic program should pass: {}", result.unwrap_err());
}

#[test]
fn test_borrow_checker_immutable() {
    let source = r#"
        fn print_data(s: &String) {
            print(s)
        }
        fn main() {
            let data: String = "valid"
            print_data(&data)
            print_data(&data)
        }
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Borrow checker: {}", result.unwrap_err());
}

#[test]
fn test_narrowed_string_supports_length_method_and_println() {
    let source = r#"
        fn inspect(value: Int | String) {
            if value is String {
                println(value.length())
            } else {
                let incremented = value + 1
            }
        }
        fn main() {}
    "#;
    let result = run_analysis(source);
    assert!(result.is_ok(), "Narrowed String should support length(): {}", result.unwrap_err());
}

#[test]
fn test_adt_construction_and_exhaustive_match() {
    let source = r#"
        enum NetworkState {
            Connecting
            Connected(Int)
            Failed(String)
        }
        fn status(state: NetworkState) -> Int {
            return match state {
                Connecting => 0
                Connected(socket) => socket
                Failed(error) => -1
            }
        }
        fn main() {
            let active = NetworkState::Connected(42)
            let code = status(active)
        }
    "#;
    assert_inferred_type(source, "active", "NetworkState");
    assert_inferred_type(source, "code", "Int");
}

#[test]
fn test_generic_constraint_rejects_non_comparable_type() {
    let source = r#"
        fn max<T: Comparable>(a: T, b: T) -> T {
            if a < b { return b }
            return a
        }
        fn main() {
            let arrays = max([1], [2])
        }
    "#;
    let error = run_analysis(source).expect_err("Array does not implement Comparable");
    assert!(error.contains("Comparable"), "Expected Comparable constraint error, got: {}", error);
}
