#![allow(dead_code)]

use crate::ast::Program;
use crate::codegen::{CodegenEngine, TargetBackend};
use crate::diagnostic::DiagnosticEngine;
use crate::ir::ProgramIR;
use crate::optimizer::OptimizerPipeline;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::typeck::TypeEnv;

pub struct CompilerOptions {
    pub target: TargetBackend,
    pub opt_level: u8,
    pub emit_ir: bool,
    pub emit_wat: bool,
    pub emit_c: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            target: TargetBackend::Bytecode,
            opt_level: 2,
            emit_ir: false,
            emit_wat: false,
            emit_c: false,
        }
    }
}

pub struct CompilerDriver {
    pub options: CompilerOptions,
    pub diagnostics: DiagnosticEngine,
}

impl CompilerDriver {
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            options,
            diagnostics: DiagnosticEngine::new(),
        }
    }

    pub fn compile_source(&mut self, source: &str, filename: &str) -> Result<String, Vec<String>> {
        // Step 1: Lexing & Parsing into AST
        let mut parser = Parser::new(source);
        let program = parser.parse_program();

        // Step 2: Name Resolution
        let mut resolver = Resolver::new();
        for stmt in &program.statements {
            let _ = stmt;
        }

        // Step 3: Type Checking & Inference
        let mut type_env = TypeEnv::new();
        let _ = &mut type_env;

        // Step 4: Codegen / Output
        let codegen = CodegenEngine::new(self.options.target);
        Ok(codegen.emit(&program))
    }
}
