use sorayunara::ast::*;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;

/// Helper to parse source string into Program AST
fn parse_src(src: &str) -> Program {
    let tokens = tokenize(src).unwrap_or_else(|e| panic!("Lexing failed for: {src}\nError: {e:?}"));
    parse(tokens).unwrap_or_else(|e| panic!("Parsing failed for: {src}\nError: {e:?}"))
}

#[test]
fn test_grammar_program_and_function_item() {
    // Grammar Rule:
    // program  ::= item*
    // item     ::= function | struct | enum | trait | impl | import
    // function ::= "fn" identifier "(" parameters? ")" ("->" type)? block
    let src = r#"
        fn calculate_sum(a: Int, b: Int) -> Int {
            return a + b
        }
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 1);

    if let StmtKind::Function {
        name,
        params,
        ret_type,
        body,
        ..
    } = &ast.statements[0].kind
    {
        assert_eq!(name, "calculate_sum");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, "a");
        assert_eq!(params[0].1, TypeNode::Int);
        assert_eq!(params[1].0, "b");
        assert_eq!(params[1].1, TypeNode::Int);
        assert_eq!(*ret_type, TypeNode::Int);
        assert_eq!(body.len(), 1);
    } else {
        panic!("Expected Function statement");
    }
}

#[test]
fn test_grammar_struct_item() {
    // Grammar Rule:
    // struct ::= "struct" identifier "{" (identifier ":" type ("," | "\n")?)* "}"
    let src = r#"
        struct UserProfile {
            id: Int,
            username: String,
            active: Bool
        }
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 1);

    if let StmtKind::StructDecl { name, fields, .. } = &ast.statements[0].kind {
        assert_eq!(name, "UserProfile");
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].0, "id");
        assert_eq!(fields[0].1, TypeNode::Int);
        assert_eq!(fields[1].0, "username");
        assert_eq!(fields[1].1, TypeNode::String);
        assert_eq!(fields[2].0, "active");
        assert_eq!(fields[2].1, TypeNode::Bool);
    } else {
        panic!("Expected StructDecl statement");
    }
}

#[test]
fn test_grammar_enum_item() {
    // Grammar Rule:
    // enum ::= "enum" identifier "{" (identifier ("(" type ")")? ("," | "\n")?)* "}"
    let src = r#"
        enum WebEvent {
            PageLoad,
            KeyPress(Int),
            Click(String)
        }
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 1);

    if let StmtKind::EnumDecl { name, variants, .. } = &ast.statements[0].kind {
        assert_eq!(name, "WebEvent");
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].0, "PageLoad");
        assert_eq!(variants[0].1, None);
        assert_eq!(variants[1].0, "KeyPress");
        assert_eq!(variants[1].1, Some(TypeNode::Int));
        assert_eq!(variants[2].0, "Click");
        assert_eq!(variants[2].1, Some(TypeNode::String));
    } else {
        panic!("Expected EnumDecl statement");
    }
}

#[test]
fn test_grammar_trait_and_impl_items() {
    // Grammar Rule:
    // trait ::= "trait" identifier "{" trait_item* "}"
    // impl  ::= "impl" (identifier "for")? type "{" impl_item* "}"
    let src = r#"
        trait Printable {
            fn format(&self) -> String
        }

        struct Document {
            title: String
        }

        impl Printable for Document {
            fn format(&self) -> String {
                return self.title
            }
        }
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 3);

    // 1. Trait verification
    if let StmtKind::TraitDecl { name, methods, .. } = &ast.statements[0].kind {
        assert_eq!(name, "Printable");
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0].name, "format");
    } else {
        panic!("Expected TraitDecl statement");
    }

    // 2. Struct verification
    assert!(matches!(
        ast.statements[1].kind,
        StmtKind::StructDecl { .. }
    ));

    // 3. Impl verification
    if let StmtKind::ImplBlock {
        trait_ref,
        target_type,
        items,
        ..
    } = &ast.statements[2].kind
    {
        assert_eq!(*trait_ref, Some(TypeNode::Custom("Printable".to_string())));
        assert_eq!(*target_type, TypeNode::Custom("Document".to_string()));
        assert_eq!(items.len(), 1);
    } else {
        panic!("Expected ImplBlock statement");
    }
}

#[test]
fn test_grammar_import_item() {
    // Grammar Rule:
    // import ::= "import" path
    let src = r#"
        import std.math
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 1);

    if let StmtKind::Import(path) = &ast.statements[0].kind {
        assert_eq!(path, "std.math");
    } else {
        panic!("Expected Import statement");
    }
}

#[test]
fn test_grammar_expression_hierarchy_and_ast_derivation() {
    // Grammar Rule:
    // expression ::= literal
    //              | identifier
    //              | function_call
    //              | binary_expression
    //              | unary_expression
    //              | array_literal
    //              | tuple_literal
    let src = r#"
        fn test_expressions() {
            let lit = 42
            let ident = lit
            let call = compute(lit, 10)
            let bin = lit * 2 + 5
            let neg = -lit
            let arr = [1, 2, 3]
            let tup = (10, 20)
        }
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 1);

    if let StmtKind::Function { body, .. } = &ast.statements[0].kind {
        assert_eq!(body.len(), 7);

        // 1. Literal
        if let StmtKind::Let { value, .. } = &body[0].kind {
            assert!(matches!(value.kind, ExprKind::Int(42)));
        }

        // 2. Identifier (Var)
        if let StmtKind::Let { value, .. } = &body[1].kind {
            assert!(matches!(&value.kind, ExprKind::Var(v) if v == "lit"));
        }

        // 3. Function Call
        if let StmtKind::Let { value, .. } = &body[2].kind {
            if let ExprKind::Call { callee, args } = &value.kind {
                assert_eq!(callee, "compute");
                assert_eq!(args.len(), 2);
            } else {
                panic!("Expected Call expression");
            }
        }

        // 4. Binary Expression (Pratt Precedence Check: (lit * 2) + 5)
        if let StmtKind::Let { value, .. } = &body[3].kind {
            if let ExprKind::Binary { left, op, right } = &value.kind {
                assert_eq!(*op, BinaryOpKind::Add);
                assert!(matches!(right.kind, ExprKind::Int(5)));
                assert!(matches!(
                    left.kind,
                    ExprKind::Binary {
                        op: BinaryOpKind::Mul,
                        ..
                    }
                ));
            } else {
                panic!("Expected Binary Add expression");
            }
        }

        // 5. Unary Expression
        if let StmtKind::Let { value, .. } = &body[4].kind {
            assert!(matches!(
                value.kind,
                ExprKind::Unary {
                    op: UnaryOpKind::Neg,
                    ..
                }
            ));
        }

        // 6. Array Literal
        if let StmtKind::Let { value, .. } = &body[5].kind {
            assert!(matches!(value.kind, ExprKind::Array(..)));
        }

        // 7. Tuple Literal
        if let StmtKind::Let { value, .. } = &body[6].kind {
            assert!(matches!(value.kind, ExprKind::Tuple(..)));
        }
    } else {
        panic!("Expected Function");
    }
}

#[test]
fn test_grammar_match_expression_ast() {
    // Grammar Rule:
    // match_expression ::= "match" expression "{" match_arm* "}"
    // match_arm        ::= pattern "=>" expression ("," | "\n")?
    let src = r#"
        fn test_match(code: Int) -> String {
            let msg = match code {
                200 => "OK",
                404 => "Not Found",
                _ => "Unknown"
            }
            return msg
        }
    "#;
    let ast = parse_src(src);
    assert_eq!(ast.statements.len(), 1);

    if let StmtKind::Function { body, .. } = &ast.statements[0].kind {
        if let StmtKind::Let { value, .. } = &body[0].kind {
            if let ExprKind::Match {
                value: target,
                arms,
            } = &value.kind
            {
                assert!(matches!(&target.kind, ExprKind::Var(v) if v == "code"));
                assert_eq!(arms.len(), 3);
                assert!(matches!(arms[0].pattern, MatchPattern::Literal(..)));
                assert!(matches!(arms[1].pattern, MatchPattern::Literal(..)));
                assert!(matches!(arms[2].pattern, MatchPattern::Wildcard));
            } else {
                panic!("Expected Match expression");
            }
        }
    }
}
