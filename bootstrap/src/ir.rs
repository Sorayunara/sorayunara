#![allow(dead_code)]

use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    PushInt(i64),
    PushFloat(f64),
    PushStr(String),
    PushChar(char),
    PushBool(bool),
    PushNull,

    PushSome,
    PushNone,
    PushOk,
    PushErr,

    Load(String),
    Store(String),

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Not,
    Neg,

    Jump(usize),
    JumpIfFalse(usize),

    Call(String, usize),
    CallExtern(String, usize),
    Spawn(String, usize),
    Await,
    Return,
    Print(usize),

    MakeArray(usize),
    MakeMap(usize),
    MakeChan,
    SendChan,
    RecvChan,
    GetIndex,
    SetIndex,
    UnwrapPayload,
    IsSome,
    IsNone,
    IsOk,
    IsErr,
    Assert,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<OpCode>,
}

impl IrFunction {
    pub fn disassemble(&self) -> String {
        let mut out = format!("--- Function: {} ({}) ---\n", self.name, self.params.join(", "));
        for (offset, op) in self.instructions.iter().enumerate() {
            out.push_str(&format!("{:04}  {:?}\n", offset, op));
        }
        out
    }
}

#[derive(Debug, Clone)]
pub struct IrExternFn {
    pub name: String,
    pub abi: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: HashMap<String, IrFunction>,
    pub externs: HashMap<String, IrExternFn>,
}

impl IrProgram {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            externs: HashMap::new(),
        }
    }
}

pub struct IrCompiler {
    functions: HashMap<String, IrFunction>,
    externs: HashMap<String, IrExternFn>,
    current_instructions: Vec<OpCode>,
    loop_context_stack: Vec<LoopContext>,
}

struct LoopContext {
    start_ip: usize,
    break_patch_ips: Vec<usize>,
}

impl IrCompiler {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            externs: HashMap::new(),
            current_instructions: Vec::new(),
            loop_context_stack: Vec::new(),
        }
    }

    pub fn compile(mut self, program: &Program) -> IrProgram {
        // Register external (FFI) symbols first so Call sites can resolve them
        for stmt in &program.statements {
            if let StmtKind::ExternBlock { abi, functions, .. } = &stmt.kind {
                for ext_fn in functions {
                    self.externs.insert(
                        ext_fn.name.clone(),
                        IrExternFn {
                            name: ext_fn.name.clone(),
                            abi: abi.clone(),
                            params: ext_fn.params.iter().map(|p| p.0.clone()).collect(),
                        },
                    );
                }
            }
        }

        let mut top_level_compiler = IrCompiler::new();
        top_level_compiler.externs = self.externs.clone();
        for stmt in &program.statements {
            match &stmt.kind {
                StmtKind::Const { .. } | StmtKind::Let { .. } | StmtKind::Comptime(_) => {
                    top_level_compiler.compile_stmt(stmt);
                }
                _ => {}
            }
        }

        for stmt in &program.statements {
            if let StmtKind::Function {
                name,
                params,
                body,
                ..
            } = &stmt.kind
            {
                let mut fn_compiler = IrCompiler::new();
                fn_compiler.externs = self.externs.clone();
                if name == "main" {
                    fn_compiler.current_instructions.extend(top_level_compiler.current_instructions.clone());
                }

                for s in body {
                    fn_compiler.compile_stmt(s);
                }

                // Safety net: always append PushNull + Return at the end of every
                // function body. If all code paths already return explicitly, these
                // instructions are unreachable dead code. But if any branch falls
                // through (e.g. else-if chains where the last instruction is a Jump),
                // this ensures the function still returns Null instead of causing a
                // stack underflow in the caller.
                if fn_compiler.current_instructions.last() != Some(&OpCode::Return) {
                    fn_compiler.current_instructions.push(OpCode::PushNull);
                    fn_compiler.current_instructions.push(OpCode::Return);
                }

                self.functions.insert(
                    name.clone(),
                    IrFunction {
                        name: name.clone(),
                        params: params.iter().map(|p| p.0.clone()).collect(),
                        instructions: fn_compiler.current_instructions,
                    },
                );
            } else if let StmtKind::TestBlock { name, body } = &stmt.kind {
                let mut test_compiler = IrCompiler::new();
                test_compiler.externs = self.externs.clone();
                for s in body {
                    test_compiler.compile_stmt(s);
                }
                if test_compiler.current_instructions.last() != Some(&OpCode::Return) {
                    test_compiler.current_instructions.push(OpCode::Return);
                }
                self.functions.insert(
                    format!("__test_{}", name),
                    IrFunction {
                        name: format!("__test_{}", name),
                        params: Vec::new(),
                        instructions: test_compiler.current_instructions,
                    },
                );
            }
        }

        IrProgram {
            functions: self.functions,
            externs: self.externs,
        }
    }

    fn emit(&mut self, op: OpCode) -> usize {
        let ip = self.current_instructions.len();
        self.current_instructions.push(op);
        ip
    }

    fn patch_jump(&mut self, ip: usize, target: usize) {
        match &mut self.current_instructions[ip] {
            OpCode::Jump(dest) | OpCode::JumpIfFalse(dest) => {
                *dest = target;
            }
            _ => panic!("Tried to patch non-jump opcode at {}", ip),
        }
    }

    fn compile_stmt(&mut self, stmt: &SpannedStmt) {
        match &stmt.kind {
            StmtKind::StructDecl { .. }
            | StmtKind::EnumDecl { .. }
            | StmtKind::TypeAlias { .. }
            | StmtKind::TraitDecl { .. }
            | StmtKind::ImplBlock { .. }
            | StmtKind::Operator { .. } => {}
            StmtKind::Function { .. } => {}
            StmtKind::LetDestructure { value, .. } => {
                self.compile_expr(value);
            }
            StmtKind::Let { name, value, .. } => {
                self.compile_expr(value);
                self.emit(OpCode::Store(name.clone()));
            }
            StmtKind::Const { name, value, .. } => {
                self.compile_expr(value);
                self.emit(OpCode::Store(name.clone()));
            }
            StmtKind::Comptime(body) => {
                for s in body {
                    self.compile_stmt(s);
                }
            }
            StmtKind::Assign { target, value } => {
                self.compile_expr(value);
                self.emit(OpCode::Store(target.clone()));
            }
            StmtKind::AssignIndex {
                target,
                index,
                value,
            } => {
                self.compile_expr(target);
                self.compile_expr(index);
                self.compile_expr(value);
                self.emit(OpCode::SetIndex);
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expr(condition);
                let jump_false_ip = self.emit(OpCode::JumpIfFalse(0));

                for s in then_branch {
                    self.compile_stmt(s);
                }

                if let Some(eb) = else_branch {
                    let jump_end_ip = self.emit(OpCode::Jump(0));
                    let else_start = self.current_instructions.len();
                    self.patch_jump(jump_false_ip, else_start);

                    for s in eb {
                        self.compile_stmt(s);
                    }
                    let end_ip = self.current_instructions.len();
                    self.patch_jump(jump_end_ip, end_ip);
                } else {
                    let end_ip = self.current_instructions.len();
                    self.patch_jump(jump_false_ip, end_ip);
                }
            }
            StmtKind::While { condition, body } => {
                let loop_start_ip = self.current_instructions.len();
                self.loop_context_stack.push(LoopContext {
                    start_ip: loop_start_ip,
                    break_patch_ips: Vec::new(),
                });

                self.compile_expr(condition);
                let jump_false_ip = self.emit(OpCode::JumpIfFalse(0));

                for s in body {
                    self.compile_stmt(s);
                }

                self.emit(OpCode::Jump(loop_start_ip));
                let loop_end_ip = self.current_instructions.len();
                self.patch_jump(jump_false_ip, loop_end_ip);

                let loop_ctx = self.loop_context_stack.pop().unwrap();
                for break_ip in loop_ctx.break_patch_ips {
                    self.patch_jump(break_ip, loop_end_ip);
                }
            }
            StmtKind::Loop { body } => {
                let loop_start_ip = self.current_instructions.len();
                self.loop_context_stack.push(LoopContext {
                    start_ip: loop_start_ip,
                    break_patch_ips: Vec::new(),
                });

                for s in body {
                    self.compile_stmt(s);
                }

                self.emit(OpCode::Jump(loop_start_ip));
                let loop_end_ip = self.current_instructions.len();

                let loop_ctx = self.loop_context_stack.pop().unwrap();
                for break_ip in loop_ctx.break_patch_ips {
                    self.patch_jump(break_ip, loop_end_ip);
                }
            }
            StmtKind::Break => {
                let break_ip = self.emit(OpCode::Jump(0));
                if let Some(ctx) = self.loop_context_stack.last_mut() {
                    ctx.break_patch_ips.push(break_ip);
                }
            }
            StmtKind::Continue => {
                if let Some(ctx) = self.loop_context_stack.last() {
                    self.emit(OpCode::Jump(ctx.start_ip));
                }
            }
            StmtKind::Return(val_opt) => {
                if let Some(val) = val_opt {
                    self.compile_expr(val);
                } else {
                    self.emit(OpCode::PushNull);
                }
                self.emit(OpCode::Return);
            }
            StmtKind::Print(args) => {
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(OpCode::Print(args.len()));
            }
            StmtKind::Assert(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Assert);
            }
            StmtKind::TestBlock { .. } => {}
            StmtKind::Expr(expr) => {
                self.compile_expr(expr);
            }
            StmtKind::Import(_) | StmtKind::Mod(_) | StmtKind::ExternBlock { .. } => {}
            StmtKind::UnsafeBlock(body) => {
                // Unsafe mode: raw pointer access. Statements are compiled
                // normally; the VM runtime keeps pointer semantics unchecked.
                for s in body {
                    self.compile_stmt(s);
                }
            }
        }
    }

    fn compile_expr(&mut self, expr: &SpannedExpr) {
        match &expr.kind {
            ExprKind::Move(inner) => {
                // Move semantics: compile the value; ownership transfer is a
                // compile-time concept so no VM opcode is needed here.
                self.compile_expr(inner);
            }
            ExprKind::UnsafeBlock(stmts) => {
                for s in stmts {
                    self.compile_stmt(s);
                }
            }
            _ => self.compile_expr_default(expr),
        }
    }

    fn compile_expr_default(&mut self, expr: &SpannedExpr) {
        match &expr.kind {
            ExprKind::Move(inner) => self.compile_expr(inner),
            ExprKind::UnsafeBlock(stmts) => {
                for s in stmts {
                    self.compile_stmt(s);
                }
            }
            ExprKind::Int(n) => {
                self.emit(OpCode::PushInt(*n));
            }
            ExprKind::Float(f) => {
                self.emit(OpCode::PushFloat(*f));
            }
            ExprKind::Str(s) => {
                self.emit(OpCode::PushStr(s.clone()));
            }
            ExprKind::Char(c) => {
                self.emit(OpCode::PushChar(*c));
            }
            ExprKind::Bool(b) => {
                self.emit(OpCode::PushBool(*b));
            }
            ExprKind::Null => {
                self.emit(OpCode::PushNull);
            }
            ExprKind::Some(inner) => {
                self.compile_expr(inner);
                self.emit(OpCode::PushSome);
            }
            ExprKind::None => {
                self.emit(OpCode::PushNone);
            }
            ExprKind::Ok(inner) => {
                self.compile_expr(inner);
                self.emit(OpCode::PushOk);
            }
            ExprKind::Err(inner) => {
                self.compile_expr(inner);
                self.emit(OpCode::PushErr);
            }
            ExprKind::Var(name) => {
                self.emit(OpCode::Load(name.clone()));
            }
            ExprKind::Borrow { expr, .. } => {
                self.compile_expr(expr);
            }
            ExprKind::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    BinaryOpKind::Add => self.emit(OpCode::Add),
                    BinaryOpKind::Sub => self.emit(OpCode::Sub),
                    BinaryOpKind::Mul => self.emit(OpCode::Mul),
                    BinaryOpKind::Div => self.emit(OpCode::Div),
                    BinaryOpKind::Mod => self.emit(OpCode::Mod),
                    BinaryOpKind::Equal => self.emit(OpCode::Equal),
                    BinaryOpKind::NotEqual => self.emit(OpCode::NotEqual),
                    BinaryOpKind::Less => self.emit(OpCode::Less),
                    BinaryOpKind::LessEqual => self.emit(OpCode::LessEqual),
                    BinaryOpKind::Greater => self.emit(OpCode::Greater),
                    BinaryOpKind::GreaterEqual => self.emit(OpCode::GreaterEqual),
                    BinaryOpKind::And => self.emit(OpCode::Mul),
                    BinaryOpKind::Or => self.emit(OpCode::Add),
                };
            }
            ExprKind::Unary { op, expr } => {
                self.compile_expr(expr);
                match op {
                    UnaryOpKind::Neg => { self.emit(OpCode::Neg); }
                    UnaryOpKind::Not => { self.emit(OpCode::Not); }
                    UnaryOpKind::Deref => {}
                };
            }
            ExprKind::Task(inner) => {
                self.compile_expr(inner);
            }
            ExprKind::Await(inner) => {
                self.compile_expr(inner);
                self.emit(OpCode::Await);
            }
            ExprKind::Spawn { callee, args } => {
                for arg in args {
                    self.compile_expr(arg);
                }
                self.emit(OpCode::Spawn(callee.clone(), args.len()));
            }
            ExprKind::MakeChan(_) => {
                self.emit(OpCode::MakeChan);
            }
            ExprKind::ChanSend { chan, value } => {
                self.compile_expr(chan);
                self.compile_expr(value);
                self.emit(OpCode::SendChan);
            }
            ExprKind::ChanRecv(chan) => {
                self.compile_expr(chan);
                self.emit(OpCode::RecvChan);
            }
            ExprKind::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg);
                }
                if callee == "__aether_method::length" {
                    self.emit(OpCode::CallExtern("strlen".to_string(), 1));
                } else if callee == "println" {
                    self.emit(OpCode::Print(args.len()));
                } else if self.externs.contains_key(callee) {
                    self.emit(OpCode::CallExtern(callee.clone(), args.len()));
                } else {
                    self.emit(OpCode::Call(callee.clone(), args.len()));
                }
            }
            ExprKind::Array(elements) => {
                for elem in elements {
                    self.compile_expr(elem);
                }
                self.emit(OpCode::MakeArray(elements.len()));
            }
            ExprKind::Tuple(items) | ExprKind::Set(items) => {
                for item in items {
                    self.compile_expr(item);
                }
                self.emit(OpCode::MakeArray(items.len()));
            }
            ExprKind::Map(entries) => {
                for (k, v) in entries {
                    self.compile_expr(k);
                    self.compile_expr(v);
                }
                self.emit(OpCode::MakeMap(entries.len()));
            }
            ExprKind::Index { target, index } => {
                self.compile_expr(target);
                self.compile_expr(index);
                self.emit(OpCode::GetIndex);
            }
            ExprKind::Dot { target, field } => {
                self.compile_expr(target);
                self.emit(OpCode::PushStr(field.clone()));
                self.emit(OpCode::GetIndex);
            }
            ExprKind::CustomBinary { left, right, .. } => {
                self.compile_expr(left);
                self.compile_expr(right);
                self.emit(OpCode::Add);
            }
            ExprKind::IsA { .. } => {
                self.emit(OpCode::PushBool(true));
            }
            ExprKind::EnumVariantConstruct { payload, .. } => {
                if let Some(payload) = payload {
                    self.compile_expr(payload);
                } else {
                    self.emit(OpCode::PushNull);
                }
            }
            ExprKind::Match { value, arms } => {
                self.compile_expr(value);
                let match_val_temp = "__match_val".to_string();
                self.emit(OpCode::Store(match_val_temp.clone()));

                let mut end_jumps = Vec::new();

                for arm in arms {
                    self.emit(OpCode::Load(match_val_temp.clone()));
                    let mut arm_jump_false = None;

                    match &arm.pattern {
                        MatchPattern::Some(var) => {
                            self.emit(OpCode::IsSome);
                            let jf = self.emit(OpCode::JumpIfFalse(0));
                            arm_jump_false = Some(jf);

                            self.emit(OpCode::Load(match_val_temp.clone()));
                            self.emit(OpCode::UnwrapPayload);
                            self.emit(OpCode::Store(var.clone()));
                        }
                        MatchPattern::None => {
                            self.emit(OpCode::IsNone);
                            let jf = self.emit(OpCode::JumpIfFalse(0));
                            arm_jump_false = Some(jf);
                        }
                        MatchPattern::Ok(var) => {
                            self.emit(OpCode::IsOk);
                            let jf = self.emit(OpCode::JumpIfFalse(0));
                            arm_jump_false = Some(jf);

                            self.emit(OpCode::Load(match_val_temp.clone()));
                            self.emit(OpCode::UnwrapPayload);
                            self.emit(OpCode::Store(var.clone()));
                        }
                        MatchPattern::Err(var) => {
                            self.emit(OpCode::IsErr);
                            let jf = self.emit(OpCode::JumpIfFalse(0));
                            arm_jump_false = Some(jf);

                            self.emit(OpCode::Load(match_val_temp.clone()));
                            self.emit(OpCode::UnwrapPayload);
                            self.emit(OpCode::Store(var.clone()));
                        }
                        MatchPattern::Var(var) => {
                            self.emit(OpCode::Store(var.clone()));
                        }
                        MatchPattern::Wildcard => {}
                        _ => {}
                    }

                    self.compile_expr(&arm.body);
                    let j_end = self.emit(OpCode::Jump(0));
                    end_jumps.push(j_end);

                    if let Some(jf) = arm_jump_false {
                        let next_arm_ip = self.current_instructions.len();
                        self.patch_jump(jf, next_arm_ip);
                    }
                }

                let final_ip = self.current_instructions.len();
                for j in end_jumps {
                    self.patch_jump(j, final_ip);
                }
            }
            ExprKind::Block(stmts) => {
                for s in stmts {
                    self.compile_stmt(s);
                }
            }
        }
    }
}

pub fn compile_to_ir(program: &Program) -> IrProgram {
    IrCompiler::new().compile(program)
}
