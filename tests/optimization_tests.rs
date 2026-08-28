use sorayunara::ir::{IrFunction, OpCode};
use sorayunara::optimizer::Optimizer;

#[test]
fn test_constant_folding_addition() {
    let mut func = IrFunction {
        name: "test_calc".to_string(),
        params: vec![],
        instructions: vec![
            OpCode::PushInt(10),
            OpCode::PushInt(20),
            OpCode::Add,
            OpCode::Return,
        ],
    };

    let mut opt = Optimizer::new();
    opt.optimize_function(&mut func);

    assert_eq!(func.instructions.len(), 2);
    assert_eq!(func.instructions[0], OpCode::PushInt(30));
    assert_eq!(func.instructions[1], OpCode::Return);
}

#[test]
fn test_constant_folding_multiplication() {
    let mut func = IrFunction {
        name: "test_mul".to_string(),
        params: vec![],
        instructions: vec![
            OpCode::PushInt(6),
            OpCode::PushInt(7),
            OpCode::Mul,
            OpCode::Return,
        ],
    };

    let mut opt = Optimizer::new();
    opt.optimize_function(&mut func);

    assert_eq!(func.instructions.len(), 2);
    assert_eq!(func.instructions[0], OpCode::PushInt(42));
    assert_eq!(func.instructions[1], OpCode::Return);
}
