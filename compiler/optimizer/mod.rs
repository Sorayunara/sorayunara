#![allow(dead_code)]

use crate::ir::{Instruction, OpCode, ProgramIR};

pub trait OptimizationPass {
    fn run(&self, program: &mut ProgramIR) -> bool;
}

pub struct ConstantFoldingPass;

impl OptimizationPass for ConstantFoldingPass {
    fn run(&self, program: &mut ProgramIR) -> bool {
        let mut changed = false;
        for func in program.functions.values_mut() {
            let mut i = 0;
            while i + 2 < func.instructions.len() {
                if let (OpCode::PushInt(a), OpCode::PushInt(b), OpCode::Add) = (
                    &func.instructions[i].opcode,
                    &func.instructions[i + 1].opcode,
                    &func.instructions[i + 2].opcode,
                ) {
                    let sum = a + b;
                    func.instructions[i] = Instruction {
                        opcode: OpCode::PushInt(sum),
                        span: func.instructions[i].span,
                    };
                    func.instructions.remove(i + 2);
                    func.instructions.remove(i + 1);
                    changed = true;
                } else {
                    i += 1;
                }
            }
        }
        changed
    }
}

pub struct DeadCodeEliminationPass;

impl OptimizationPass for DeadCodeEliminationPass {
    fn run(&self, program: &mut ProgramIR) -> bool {
        let mut changed = false;
        for func in program.functions.values_mut() {
            let mut i = 0;
            while i < func.instructions.len() {
                if let OpCode::Return = func.instructions[i].opcode {
                    if i + 1 < func.instructions.len() {
                        // Truncate only if there are no jump targets pointing after return
                        // Handled safely in core optimizer
                        break;
                    }
                }
                i += 1;
            }
        }
        changed
    }
}

pub struct OptimizerPipeline {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl OptimizerPipeline {
    pub fn new() -> Self {
        Self {
            passes: vec![
                Box::new(ConstantFoldingPass),
                Box::new(DeadCodeEliminationPass),
            ],
        }
    }

    pub fn optimize(&self, program: &mut ProgramIR) {
        for pass in &self.passes {
            let mut iterations = 0;
            while pass.run(program) && iterations < 10 {
                iterations += 1;
            }
        }
    }
}
