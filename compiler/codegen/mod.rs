#![allow(dead_code)]

use crate::ast::Program;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetBackend {
    Llvm,
    C99,
    Wasm,
    Bytecode,
}

pub struct CodegenEngine {
    pub target: TargetBackend,
}

impl CodegenEngine {
    pub fn new(target: TargetBackend) -> Self {
        Self { target }
    }

    pub fn emit(&self, program: &Program) -> String {
        match self.target {
            TargetBackend::Llvm => crate::llvm_backend::emit_llvm_ir(program),
            TargetBackend::C99 => crate::codegen::emit_c(program),
            TargetBackend::Wasm => crate::wasm_backend::emit_wat(program),
            TargetBackend::Bytecode => format!("; Bytecode representation for {} statements", program.statements.len()),
        }
    }
}
