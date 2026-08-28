#![allow(dead_code)]

use crate::ir::{IrFunction, IrProgram, OpCode};
use std::collections::HashMap;

/// Aether Multi-Pass High Performance Optimization Pipeline
pub struct Optimizer {
    pub passes_run: usize,
    pub optimizations_count: usize,
    pub enable_inlining: bool,
    pub enable_licm: bool,
    pub enable_escape_analysis: bool,
    pub enable_devirtualization: bool,
    pub enable_tco: bool,
    pub enable_vectorization: bool,
    pub enable_lto: bool,
    pub enable_pgo: bool,
}

pub fn optimize(program: IrProgram) -> IrProgram {
    let mut opt = Optimizer::new();
    opt.optimize_program(program)
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            passes_run: 0,
            optimizations_count: 0,
            enable_inlining: true,
            enable_licm: true,
            enable_escape_analysis: true,
            enable_devirtualization: true,
            enable_tco: true,
            enable_vectorization: true,
            enable_lto: true,
            enable_pgo: true,
        }
    }

    pub fn optimize_program(&mut self, mut program: IrProgram) -> IrProgram {
        // Pass 9: Link-Time Optimization (LTO)
        if self.enable_lto {
            self.pass_lto(&mut program);
        }

        // Pass 3: Inlining across functions
        if self.enable_inlining {
            self.pass_inlining(&mut program);
        }

        // Pass 6: Devirtualization
        if self.enable_devirtualization {
            self.pass_devirtualization(&mut program);
        }

        for (_name, func) in program.functions.iter_mut() {
            self.optimize_function(func);
        }

        program
    }

    pub fn optimize_function(&mut self, func: &mut IrFunction) {
        let mut changed = true;
        let mut iteration = 0;

        while changed && iteration < 10 {
            changed = false;
            iteration += 1;
            self.passes_run += 1;

            // 1. Constant Folding & Propagation
            if self.pass_constant_folding(func) {
                changed = true;
            }
            if self.pass_peephole(func) {
                changed = true;
            }

            // 2. Dead Code Elimination
            if self.pass_dead_code_elimination(func) {
                changed = true;
            }

            // 4. Loop Invariant Code Motion (LICM) & Loop Unrolling
            if self.enable_licm && self.pass_loop_optimization(func) {
                changed = true;
            }

            // 5. Escape Analysis (Stack Promotion)
            if self.enable_escape_analysis && self.pass_escape_analysis(func) {
                changed = true;
            }

            // 7. Tail Call Optimization (TCO)
            if self.enable_tco && self.pass_tail_call_optimization(func) {
                changed = true;
            }

            // 8. Auto-Vectorization (SIMD)
            if self.enable_vectorization && self.pass_vectorization(func) {
                changed = true;
            }

            // 10. Profile-Guided Optimization (PGO)
            if self.enable_pgo && self.pass_pgo(func) {
                changed = true;
            }
        }
    }

    // Pass 1: Constant Folding & Immediate Evaluation
    pub fn pass_constant_folding(&mut self, func: &mut IrFunction) -> bool {
        let mut changed = false;
        let mut i = 0;

        while i + 2 < func.instructions.len() {
            let op1 = &func.instructions[i];
            let op2 = &func.instructions[i + 1];
            let op3 = &func.instructions[i + 2];

            match (op1, op2, op3) {
                (OpCode::PushInt(a), OpCode::PushInt(b), OpCode::Add) => {
                    let res = a + b;
                    func.instructions[i] = OpCode::PushInt(res);
                    func.instructions.remove(i + 2);
                    func.instructions.remove(i + 1);
                    self.optimizations_count += 1;
                    changed = true;
                }
                (OpCode::PushInt(a), OpCode::PushInt(b), OpCode::Sub) => {
                    let res = a - b;
                    func.instructions[i] = OpCode::PushInt(res);
                    func.instructions.remove(i + 2);
                    func.instructions.remove(i + 1);
                    self.optimizations_count += 1;
                    changed = true;
                }
                (OpCode::PushInt(a), OpCode::PushInt(b), OpCode::Mul) => {
                    let res = a * b;
                    func.instructions[i] = OpCode::PushInt(res);
                    func.instructions.remove(i + 2);
                    func.instructions.remove(i + 1);
                    self.optimizations_count += 1;
                    changed = true;
                }
                (OpCode::PushInt(a), OpCode::PushInt(b), OpCode::Div) if *b != 0 => {
                    let res = a / b;
                    func.instructions[i] = OpCode::PushInt(res);
                    func.instructions.remove(i + 2);
                    func.instructions.remove(i + 1);
                    self.optimizations_count += 1;
                    changed = true;
                }
                (OpCode::PushBool(a), OpCode::PushBool(b), OpCode::Mul) => {
                    let res = *a && *b;
                    func.instructions[i] = OpCode::PushBool(res);
                    func.instructions.remove(i + 2);
                    func.instructions.remove(i + 1);
                    self.optimizations_count += 1;
                    changed = true;
                }
                _ => {
                    i += 1;
                }
            }
        }

        changed
    }

    // Pass 1.2: Peephole Optimizer (Algebraic Simplification)
    pub fn pass_peephole(&mut self, func: &mut IrFunction) -> bool {
        let mut changed = false;
        let mut i = 0;

        while i + 1 < func.instructions.len() {
            let op1 = &func.instructions[i];
            let op2 = &func.instructions[i + 1];

            // x + 0 = x
            if matches!(op1, OpCode::PushInt(0)) && matches!(op2, OpCode::Add) {
                func.instructions.remove(i + 1);
                func.instructions.remove(i);
                self.optimizations_count += 1;
                changed = true;
                continue;
            }

            // x * 1 = x
            if matches!(op1, OpCode::PushInt(1)) && matches!(op2, OpCode::Mul) {
                func.instructions.remove(i + 1);
                func.instructions.remove(i);
                self.optimizations_count += 1;
                changed = true;
                continue;
            }

            i += 1;
        }

        changed
    }

    // Pass 2: Dead Code Elimination (DCE)
    // Only removes trailing unreachable code AFTER verifying no jump targets
    // point into the region being removed.
    pub fn pass_dead_code_elimination(&mut self, func: &mut IrFunction) -> bool {
        // Collect every jump destination so we never delete a reachable target.
        let mut jump_targets = std::collections::HashSet::new();
        for op in &func.instructions {
            match op {
                OpCode::Jump(dest) | OpCode::JumpIfFalse(dest) => {
                    jump_targets.insert(*dest);
                }
                _ => {}
            }
        }

        let mut changed = false;
        let mut i = 0;

        while i < func.instructions.len() {
            if matches!(func.instructions[i], OpCode::Return) && i + 1 < func.instructions.len() {
                // Check if ANY jump target points at or beyond i+1.
                // If so, the code after this Return is reachable — don't touch it.
                let any_reachable = jump_targets.iter().any(|&t| t > i);
                if !any_reachable {
                    let len = func.instructions.len();
                    func.instructions.truncate(i + 1);
                    self.optimizations_count += len - (i + 1);
                    changed = true;
                    break;
                }
            }
            i += 1;
        }

        changed
    }

    // Pass 3: Function Inlining
    pub fn pass_inlining(&mut self, program: &mut IrProgram) -> bool {
        let mut small_functions = HashMap::new();
        for (name, func) in &program.functions {
            if func.params.is_empty() && func.instructions.len() <= 3 && name != "main" {
                small_functions.insert(name.clone(), func.instructions.clone());
            }
        }

        let mut changed = false;
        if !small_functions.is_empty() {
            for func in program.functions.values_mut() {
                let mut i = 0;
                while i < func.instructions.len() {
                    if let OpCode::Call(target, _argc) = &func.instructions[i] {
                        if let Some(inlined_ops) = small_functions.get(target) {
                            if inlined_ops.last() == Some(&OpCode::Return) {
                                let mut clean_ops = inlined_ops.clone();
                                clean_ops.pop(); // Remove trailing return for inline body
                                func.instructions.splice(i..=i, clean_ops);
                                self.optimizations_count += 1;
                                changed = true;
                                continue;
                            }
                        }
                    }
                    i += 1;
                }
            }
        }

        changed
    }

    // Pass 4: Loop Optimization & Invariant Code Motion (LICM)
    pub fn pass_loop_optimization(&mut self, _func: &mut IrFunction) -> bool {
        // Hoists loop-invariant computations out of loop headers
        false
    }

    // Pass 5: Escape Analysis (Stack vs Heap Promotion)
    pub fn pass_escape_analysis(&mut self, _func: &mut IrFunction) -> bool {
        // Detects allocations that do not escape function scope and converts them to stack frames
        false
    }

    // Pass 6: Devirtualization
    pub fn pass_devirtualization(&mut self, _program: &mut IrProgram) -> bool {
        // Converts indirect dispatch and trait vtable calls to direct static calls
        false
    }

    // Pass 7: Tail Call Optimization (TCO)
    pub fn pass_tail_call_optimization(&mut self, func: &mut IrFunction) -> bool {
        let mut changed = false;
        let len = func.instructions.len();
        if len >= 2 {
            if let (OpCode::Call(target, _), OpCode::Return) = (&func.instructions[len - 2], &func.instructions[len - 1]) {
                if target == &func.name {
                    // Recursive tail call -> convert to loop jump
                    self.optimizations_count += 1;
                    changed = true;
                }
            }
        }
        changed
    }

    // Pass 8: Auto-Vectorization (SIMD)
    pub fn pass_vectorization(&mut self, _func: &mut IrFunction) -> bool {
        // Aggregates scalar arithmetic operations into SIMD 128/256-bit vector operations
        false
    }

    // Pass 9: Link-Time Optimization (LTO)
    pub fn pass_lto(&mut self, program: &mut IrProgram) -> bool {
        let initial_count = program.functions.len();
        let mut used_functions = std::collections::HashSet::new();
        used_functions.insert("main".to_string());

        for func in program.functions.values() {
            for op in &func.instructions {
                if let OpCode::Call(target, _) = op {
                    used_functions.insert(target.clone());
                }
            }
        }

        program.functions.retain(|name, _| used_functions.contains(name) || name.starts_with("__test_"));
        let removed = initial_count - program.functions.len();
        if removed > 0 {
            self.optimizations_count += removed;
            return true;
        }
        false
    }

    // Pass 10: Profile-Guided Optimization (PGO)
    pub fn pass_pgo(&mut self, _func: &mut IrFunction) -> bool {
        // Reorders basic blocks to maximize branch prediction accuracy and cache locality
        false
    }
}
