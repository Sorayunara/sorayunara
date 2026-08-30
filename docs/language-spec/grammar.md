# 📐 Sorayunara Formal Language Specification: Grammar (EBNF & AST Mapping)

**Document Version**: 1.0.0  
**Specification Status**: Normative Reference Standard  
**File Identifier**: `docs/language-spec/grammar.md`  
**Execution Pipeline**: $\text{Source Code } (.sora) \xrightarrow{\text{Lexer}} \text{Token Stream} \xrightarrow{\text{Pratt Parser}} \text{AST Nodes}$

---

## 1. High-Level Program & Item Grammar

A Sorayunara program is a sequence of deterministic top-level items:

```ebnf
program        ::= item*
item           ::= function
                 | struct
                 | enum
                 | trait
                 | impl
                 | import
                 | const_decl
                 | statement
```

### 1.1 Item Definitions & AST Production Rules

```ebnf
function       ::= [ "async" ] [ "unsafe" ] "fn" identifier [ generic_params ]
                   "(" parameters? ")" [ "->" type ] where_clause? 
                   ( block | "=>" expression | "=" expression )

parameters     ::= parameter ( "," parameter )* [ "," ]
parameter      ::= [ "mut" ] [ "&" [ "mut" ] ] identifier [ ":" type ] [ "=" expression ]

struct         ::= "struct" identifier [ generic_params ] where_clause?
                   "{" struct_fields? "}"

struct_fields  ::= struct_field ( ( "," | "\n" ) struct_field )* [ "," ]
struct_field   ::= [ "pub" ] [ "mut" ] identifier ":" type [ "=" expression ]

enum           ::= "enum" identifier [ generic_params ] where_clause?
                   "{" enum_variants? "}"

enum_variants  ::= enum_variant ( ( "," | "\n" ) enum_variant )* [ "," ]
enum_variant   ::= identifier [ "(" tuple_types ")" | "{" struct_fields "}" | "=" expression ]

trait          ::= "trait" identifier [ generic_params ] [ ":" trait_bounds ] where_clause?
                   "{" trait_item* "}"

trait_item     ::= function_sig | function | type_alias | const_decl

impl           ::= "impl" [ generic_params ] [ identifier "for" ] type where_clause?
                   "{" impl_item* "}"

impl_item      ::= function | type_alias | const_decl

import         ::= "import" import_path [ "as" identifier ]
import_path    ::= identifier ( "." identifier )*
```

---

## 2. Expressions & Precedence Hierarchy (Pratt Parsing)

Every expression evaluates deterministically to an AST `SpannedExpr`:

```ebnf
expression     ::= pipeline_expr

pipeline_expr  ::= logical_or ( "|>" logical_or )*
logical_or     ::= logical_and ( "||" logical_and )*
logical_and    ::= bitwise_or ( "&&" bitwise_or )*
bitwise_or     ::= bitwise_xor ( "|" bitwise_xor )*
bitwise_xor    ::= bitwise_and ( "^" bitwise_and )*
bitwise_and    ::= equality ( "&" equality )*
equality       ::= relational ( ( "==" | "!=" | "is" ) relational )*
relational     ::= shift ( ( "<" | "<=" | ">" | ">=" | "in" ) shift )*
shift          ::= additive ( ( "<<" | ">>" ) additive )*
additive       ::= multiplicative ( ( "+" | "-" ) multiplicative )*
multiplicative ::= unary ( ( "*" | "/" | "%" | "**" ) unary )*

unary          ::= ( "-" | "!" | "~" | "&" [ "mut" ] | "*" | "await" | "move" ) unary
                 | postfix

postfix        ::= primary (
                     "(" arguments? ")"
                   | "[" expression "]"
                   | "." identifier
                   | "?." identifier
                   | "??" expression
                   | "?"
                   | "!"
                   )*

primary        ::= literal
                 | identifier
                 | function_call
                 | binary_expression
                 | match_expression
                 | block
                 | if_expression
                 | while_expression
                 | loop_expression
                 | spawn_expression
                 | "(" expression ")"
                 | array_literal
                 | map_literal
```

---

## 3. Pattern Matching Grammar

```ebnf
match_expression ::= "match" expression "{" match_arm* "}"
match_arm        ::= pattern [ "if" expression ] "=>" ( expression | block ) [ "," | "\n" ]

pattern          ::= wildcard_pattern
                   | literal_pattern
                   | identifier_pattern
                   | tuple_pattern
                   | struct_pattern
                   | enum_pattern
                   | slice_pattern

wildcard_pattern ::= "_"
literal_pattern  ::= integer_literal | float_literal | string_literal | bool_literal
identifier_pattern ::= identifier
tuple_pattern    ::= "(" pattern ( "," pattern )* ")"
struct_pattern   ::= identifier "{" ( identifier [ ":" pattern ] )* "}"
enum_pattern     ::= identifier [ "." identifier ] [ "(" pattern* ")" ]
slice_pattern    ::= "[" pattern* [ ".." [ identifier ] ] "]"
```

---

## 4. Statements & Control Flow

```ebnf
statement      ::= let_statement
                 | assignment_statement
                 | return_statement
                 | guard_statement
                 | defer_statement
                 | break_statement
                 | continue_statement
                 | expression_statement

let_statement  ::= ( "let" [ "mut" ] | "mut" ) pattern [ ":" type ] ( "=" | ":=" ) expression
assignment     ::= lvalue assign_op expression
assign_op      ::= "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>="
return_stmt    ::= "return" [ expression ]
guard_stmt     ::= "guard" expression "else" block
defer_stmt     ::= "defer" ( expression | block )
block          ::= "{" statement* [ expression ] "}"
```

---

## 5. Formal AST Mapping Table

| Grammar Rule | Token Sequence | Concrete AST Node ([`ast.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/bootstrap/src/ast.rs)) |
|---|---|---|
| `program` | `item*` | `ast::Program { statements: Vec<SpannedStmt> }` |
| `function` | `fn foo(...) -> T { ... }` | `StmtKind::Function { name, params, ret_type, body }` |
| `struct` | `struct Foo { x: Int }` | `StmtKind::Struct { name, fields, .. }` |
| `enum` | `enum Bar { A, B(Int) }` | `StmtKind::Enum { name, variants, .. }` |
| `trait` | `trait Printable { ... }` | `StmtKind::Trait { name, methods, .. }` |
| `impl` | `impl Trait for Target { ... }` | `StmtKind::Impl { trait_name, target_type, items }` |
| `import` | `import std.net` | `StmtKind::Import { path, .. }` |
| `let_stmt` | `let mut x: Int = 10` | `StmtKind::Let { pattern, type_annot, value, is_mut }` |
| `binary_expr`| `a + b`, `x * y` | `ExprKind::Binary { left, op, right }` |
| `match_expr` | `match v { ... }` | `ExprKind::Match { value, arms }` |
| `call_expr` | `compute(a, b)` | `ExprKind::Call { callee, args }` |
| `block` | `{ stmt; expr }` | `ExprKind::Block(Vec<SpannedStmt>)` |

---

## 6. Test Verification

Sistem grammar ini tervalidasi secara deterministik dalam test suite kompiler:
- **Test File**: [`tests/grammar_verification_tests.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/tests/grammar_verification_tests.rs) & [`tests/parser_tests.rs`](file:///c:/Users/muham/OneDrive/Documents/NANXIAN/tests/parser_tests.rs)
- **Eksekusi Verifikasi**: `cargo test --test grammar_verification_tests`
