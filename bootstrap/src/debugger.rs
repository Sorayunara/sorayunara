#![allow(dead_code)]

use crate::ir::IrProgram;
use crate::vm::{CallFrame, Value, VirtualMachine};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum DebugCommand {
    Step,
    Next,
    Continue,
    Breakpoint(String),
    PrintVar(String),
    ShowLocals,
    ShowStack,
    ShowMemory,
    ShowThreads,
    Quit,
}

pub struct DebugSession {
    pub vm: VirtualMachine,
    pub breakpoints: HashSet<String>,
    pub call_stack_depth: usize,
    pub paused: bool,
    pub is_running: bool,
}

impl DebugSession {
    pub fn new(program: IrProgram) -> Self {
        let mut vm = VirtualMachine::new(program.clone());
        let main_fn = program.functions.get("main").cloned();
        if let Some(m) = main_fn {
            vm.call_stack.push(CallFrame {
                fn_name: m.name,
                ip: 0,
                locals: HashMap::new(),
            });
        }
        Self {
            vm,
            breakpoints: HashSet::new(),
            call_stack_depth: 1,
            paused: true,
            is_running: true,
        }
    }

    pub fn add_breakpoint(&mut self, target: &str) {
        self.breakpoints.insert(target.to_string());
    }

    pub fn remove_breakpoint(&mut self, target: &str) {
        self.breakpoints.remove(target);
    }

    pub fn get_locals(&self) -> HashMap<String, Value> {
        if let Some(frame) = self.vm.call_stack.last() {
            frame.locals.clone()
        } else {
            HashMap::new()
        }
    }

    pub fn get_call_stack(&self) -> Vec<String> {
        self.vm
            .call_stack
            .iter()
            .map(|f| format!("{} (ip: {})", f.fn_name, f.ip))
            .collect()
    }

    pub fn step_instruction(&mut self) -> Result<Option<Value>, String> {
        if self.vm.call_stack.is_empty() {
            self.is_running = false;
            return Ok(Some(Value::Null));
        }

        let frame_idx = self.vm.call_stack.len() - 1;
        let fn_name = self.vm.call_stack[frame_idx].fn_name.clone();
        let ip = self.vm.call_stack[frame_idx].ip;

        let ir_fn = self
            .vm
            .program
            .functions
            .get(&fn_name)
            .ok_or_else(|| format!("Function '{}' not found", fn_name))?;

        if ip >= ir_fn.instructions.len() {
            self.vm.call_stack.pop();
            if self.vm.call_stack.is_empty() {
                self.is_running = false;
                return Ok(Some(Value::Null));
            }
            return Ok(None);
        }

        self.vm.call_stack[frame_idx].ip += 1;
        Ok(None)
    }
}
