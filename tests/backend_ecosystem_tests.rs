use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;
use sorayunara::vm::execute_ir;

#[test]
fn test_backend_ecosystem_server_flow() {
    let source = r#"
        fn main() -> Int {
            let server: Int = create_server(8080)
            let db: Int = pg_connect("postgres://admin:secret@localhost:5432/aether_db")
            let users_json: String = pg_query(db, "SELECT id, username, email FROM users")
            let payload: String = json_encode(users_json)
            let token: String = jwt_sign("admin", "supersecretkey")
            let secure_hash: String = sha256("password123")
            print("Server port: ", server)
            print("Server payload: ", payload)
            print("Auth token: ", token)
            print("Secure hash: ", secure_hash)
            return 0
        }
    "#;
    let std_http = std::fs::read_to_string("std/http.sora").unwrap();
    let std_pg = std::fs::read_to_string("std/postgres.sora").unwrap();
    let std_json = std::fs::read_to_string("std/json.sora").unwrap();
    let std_jwt = std::fs::read_to_string("std/jwt.sora").unwrap();
    let std_crypto = std::fs::read_to_string("std/crypto.sora").unwrap();

    let combined_source = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        std_http, std_pg, std_json, std_jwt, std_crypto, source
    );

    let tokens = tokenize(&combined_source).unwrap();
    let program = parse(tokens).unwrap();
    if let Err(diag) = check_semantics(&program) {
        eprintln!("{}", diag.render_all("combined.sora", &combined_source));
        panic!("Semantic check failed");
    }

    let ir = compile_to_ir(&program);
    let result = execute_ir(ir);
    assert!(result.is_ok());
}
