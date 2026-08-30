use sorayunara::debugger::DebugSession;
use sorayunara::ir::compile_to_ir;
use sorayunara::lexer::tokenize;
use sorayunara::parser::parse;
use sorayunara::profiler::Profiler;

#[test]
fn test_debugger_session_breakpoints_and_step() {
    let source = r#"
        fn compute() -> Int {
            let a: Int = 10
            let b: Int = 20
            return a + b
        }

        fn main() -> Int {
            let res: Int = compute()
            return res
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);

    let mut session = DebugSession::new(ir);
    session.add_breakpoint("compute");
    assert!(session.breakpoints.contains("compute"));

    let stack = session.get_call_stack();
    assert!(!stack.is_empty());
    assert!(stack[0].contains("main"));

    // Step instruction
    let step_res = session.step_instruction();
    assert!(step_res.is_ok());
}

#[test]
fn test_profiler_and_trace_execution() {
    let source = r#"
        fn square(x: Int) -> Int {
            return x * x
        }

        fn main() -> Int {
            let val: Int = square(5)
            return val
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    let ir = compile_to_ir(&program);

    let profiler = Profiler::new(ir);
    let (val, profiles) = profiler.profile().unwrap();
    assert_eq!(val, sorayunara::vm::Value::Int(25));
    assert!(profiles.contains_key("main"));
    assert!(profiles.contains_key("square"));

    let trace_logs = profiler.trace().unwrap();
    assert!(!trace_logs.is_empty());
    assert!(
        trace_logs
            .iter()
            .any(|l| l.contains("[TRACE] Function: main"))
    );
}
