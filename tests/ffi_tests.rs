use sorayunara::ast::*;
use sorayunara::codegen::emit_c;
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

#[test]
fn test_ffi_extern_c_block_parsing_and_typechecking() {
    let source = r#"
        extern "C" {
            fn sqlite3_open(filename: String) -> Int
            fn sqlite3_close(db: Int) -> Int
            fn puts(s: String) -> Int
        }

        fn main() -> Int {
            let res: Int = puts("Connecting to database...")
            let status: Int = sqlite3_open("production.db")
            return status
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();

    assert!(program.statements.iter().any(|s| match &s.kind {
        StmtKind::ExternBlock { abi, functions, .. } => {
            abi == "C" && functions.len() == 3 && functions[0].name == "sqlite3_open"
        }
        _ => false,
    }));

    assert!(check_semantics(&program).is_ok());
}

#[test]
fn test_ffi_llvm_ir_foreign_declarations() {
    let source = r#"
        extern "C" {
            fn sqlite3_open(filename: String) -> Int
            fn cos(x: Float) -> Float
        }

        fn main() -> Int {
            return sqlite3_open("test.db")
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let llvm_ir = emit_llvm_ir(&program);

    assert!(llvm_ir.contains("declare i64 @sqlite3_open(i8*)"));
    assert!(llvm_ir.contains("declare double @cos(double)"));
}

#[test]
fn test_ffi_c_transpilation_headers() {
    let source = r#"
        extern "C" {
            fn sqlite3_open(filename: String) -> Int
        }

        fn main() -> Int {
            return sqlite3_open("test.db")
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let c_code = emit_c(&program);

    assert!(c_code.contains("long long sqlite3_open(const char* filename);"));
}
