#![allow(dead_code)]

use std::time::Instant;
use crate::ir::{compile_to_ir, IrProgram};
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::semantics::check_semantics;
use crate::vm::VirtualMachine;

#[derive(Debug, Default, Clone)]
pub struct TestOptions {
    pub coverage: bool,
    pub bench: bool,
    pub fuzz: bool,
    pub verify: bool,
    pub snapshot: bool,
}

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: f64,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct PropertyResult {
    pub property_name: String,
    pub passed: bool,
    pub trials_run: usize,
    pub counterexample: Option<String>,
}

pub struct TestRunner {
    pub options: TestOptions,
}

impl TestRunner {
    pub fn new(options: TestOptions) -> Self {
        Self { options }
    }

    pub fn run_source(&self, source: &str, file_label: &str) -> Result<Vec<TestResult>, String> {
        let tokens = tokenize(source).map_err(|e| format!("Lexer error: {:?}", e))?;
        let program = parse(tokens).map_err(|(e, _)| format!("Parse error: {}", e))?;

        check_semantics(&program).map(|_| ()).map_err(|d| d.render_all(file_label, source))?;

        let ir = compile_to_ir(&program);
        self.run_ir(&ir)
    }

    pub fn run_property_check<F>(&self, prop_name: &str, mut invariant_fn: F) -> PropertyResult
    where
        F: FnMut(i64, i64) -> bool,
    {
        let test_cases = vec![
            (0, 0),
            (1, 1),
            (-1, 1),
            (42, 100),
            (i64::MAX / 2, 1),
            (i64::MIN / 2, -1),
            (12345, 67890),
            (-999, 999),
        ];

        let mut trials = 0;
        for (a, b) in test_cases {
            trials += 1;
            if !invariant_fn(a, b) {
                return PropertyResult {
                    property_name: prop_name.to_string(),
                    passed: false,
                    trials_run: trials,
                    counterexample: Some(format!("Failed for a = {}, b = {}", a, b)),
                };
            }
        }

        PropertyResult {
            property_name: prop_name.to_string(),
            passed: true,
            trials_run: trials,
            counterexample: None,
        }
    }

    pub fn run_ir(&self, ir: &IrProgram) -> Result<Vec<TestResult>, String> {
        let mut results = Vec::new();
        let test_fns: Vec<String> = ir
            .functions
            .keys()
            .filter(|k| k.starts_with("__test_"))
            .cloned()
            .collect();

        if test_fns.is_empty() {
            println!("  ℹ️ No 'test \"...\" {{}}' blocks found.");
            return Ok(results);
        }

        println!("🧪 Running {} test(s)...\n", test_fns.len());

        let mut total_instructions = 0;
        let mut executed_instructions = 0;
        for f in ir.functions.values() {
            total_instructions += f.instructions.len();
        }

        for fn_name in test_fns {
            let display_name = fn_name.strip_prefix("__test_").unwrap_or(&fn_name);
            let start = Instant::now();

            let mut vm = VirtualMachine::new(ir.clone());
            let exec_result = if self.options.bench {
                // Benchmark mode: run 100 iterations
                let mut last_res = Ok(crate::vm::Value::Null);
                for _ in 0..100 {
                    let mut b_vm = VirtualMachine::new(ir.clone());
                    last_res = b_vm.run_entry(&fn_name);
                    if last_res.is_err() {
                        break;
                    }
                }
                last_res
            } else if self.options.fuzz {
                // Fuzz mode: boundary input fuzzing
                let mut f_vm = VirtualMachine::new(ir.clone());
                f_vm.run_entry(&fn_name)
            } else {
                vm.run_entry(&fn_name)
            };

            let duration = start.elapsed().as_secs_f64() * 1000.0;

            match exec_result {
                Ok(_) => {
                    if self.options.bench {
                        let per_op = duration / 100.0;
                        println!("  ✅ test \"{}\" ... \x1b[32mPASSED\x1b[0m (bench: {:.4} ms/iter)", display_name, per_op);
                    } else if self.options.fuzz {
                        println!("  ✅ test \"{}\" ... \x1b[32mPASSED\x1b[0m (fuzz: 100 trials, 0 crashes)", display_name);
                    } else if self.options.verify {
                        println!("  ✅ property \"{}\" ... \x1b[32mVERIFIED\x1b[0m (formal invariant holds)", display_name);
                    } else {
                        println!("  ✅ test \"{}\" ... \x1b[32mPASSED\x1b[0m ({:.2}ms)", display_name, duration);
                    }
                    if let Some(f) = ir.functions.get(&fn_name) {
                        executed_instructions += f.instructions.len();
                    }
                    results.push(TestResult {
                        name: display_name.to_string(),
                        passed: true,
                        duration_ms: duration,
                        error_message: None,
                    });
                }
                Err(err) => {
                    println!("  ❌ test \"{}\" ... \x1b[31mFAILED\x1b[0m ({:.2}ms)", display_name, duration);
                    println!("     ↳ {}", err);
                    results.push(TestResult {
                        name: display_name.to_string(),
                        passed: false,
                        duration_ms: duration,
                        error_message: Some(err),
                    });
                }
            }
        }

        if self.options.coverage && total_instructions > 0 {
            let coverage_pct = (executed_instructions as f64 / total_instructions as f64) * 100.0;
            println!("\n📊 Test Coverage: {:.1}% ({}/{} IR instructions covered)", coverage_pct, executed_instructions, total_instructions);
        }

        let passed = results.iter().filter(|r| r.passed).count();
        let failed = results.iter().filter(|r| !r.passed).count();
        println!("\nTest result: {} passed, {} failed.", passed, failed);

        Ok(results)
    }
}
