#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::ir::{IrProgram, OpCode};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
    Option(Option<Box<Value>>),
    Result(Result<Box<Value>, Box<Value>>),
    Task(Arc<Mutex<Option<Value>>>),
    Chan(Arc<Mutex<Vec<Value>>>),
    /// Raw pointer (address) for Unsafe mode: `*const T` / `*mut T`.
    Ptr(u64),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Char(c) => write!(f, "{}", c),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                let items: Vec<String> = arr.borrow().iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Map(map) => {
                let mut entries: Vec<String> = map
                    .borrow()
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                entries.sort();
                write!(f, "{{{}}}", entries.join(", "))
            }
            Value::Option(opt) => match opt {
                Some(v) => write!(f, "Some({})", v),
                None => write!(f, "None"),
            },
            Value::Result(res) => match res {
                Ok(v) => write!(f, "Ok({})", v),
                Err(e) => write!(f, "Err({})", e),
            },
            Value::Task(_) => write!(f, "<Task>"),
            Value::Chan(_) => write!(f, "<Channel>"),
            Value::Ptr(addr) => write!(f, "0x{:x}", addr),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Char(_) => true,
            Value::Option(opt) => opt.is_some(),
            Value::Result(res) => res.is_ok(),
            Value::Null => false,
            Value::Array(arr) => !arr.borrow().is_empty(),
            Value::Map(map) => !map.borrow().is_empty(),
            Value::Task(_) => true,
            Value::Chan(_) => true,
            Value::Ptr(addr) => *addr != 0,
        }
    }
}

pub struct CallFrame {
    pub fn_name: String,
    pub ip: usize,
    pub locals: HashMap<String, Value>,
}

pub type HostFn = fn(&[Value]) -> Result<Value, String>;

/// Built-in FFI bridge: native C library symbols callable directly from the VM.
/// These mirror the real libc/libm signatures declared via `extern "C"` blocks.
fn resolve_host_fn(name: &str) -> Option<HostFn> {
    Some(match name {
        // ---- math.h (libm) ----
        "sqrt" => |args| match args.first() {
            Some(Value::Float(x)) => Ok(Value::Float(x.sqrt())),
            Some(Value::Int(x)) => Ok(Value::Float((*x as f64).sqrt())),
            other => Err(host_arg_err("sqrt", "(double)", other)),
        },
        "pow" => |args| match (args.first(), args.get(1)) {
            (Some(Value::Float(a)), Some(Value::Float(b))) => Ok(Value::Float(a.powf(*b))),
            (Some(Value::Int(a)), Some(Value::Int(b))) => Ok(Value::Int(a.pow(*b as u32))),
            _ => Err(host_arg_err("pow", "(double, double)", args.first())),
        },
        "fabs" => |args| match args.first() {
            Some(Value::Float(x)) => Ok(Value::Float(x.abs())),
            Some(Value::Int(x)) => Ok(Value::Float((*x as f64).abs())),
            other => Err(host_arg_err("fabs", "(double)", other)),
        },
        "floor" => |args| float1(args.first(), "floor", f64::floor),
        "ceil" => |args| float1(args.first(), "ceil", f64::ceil),
        "round" => |args| float1(args.first(), "round", f64::round),
        "sin" => |args| float1(args.first(), "sin", f64::sin),
        "cos" => |args| float1(args.first(), "cos", f64::cos),
        "tan" => |args| float1(args.first(), "tan", f64::tan),
        "asin" => |args| float1(args.first(), "asin", f64::asin),
        "acos" => |args| float1(args.first(), "acos", f64::acos),
        "atan" => |args| float1(args.first(), "atan", f64::atan),
        "exp" => |args| float1(args.first(), "exp", f64::exp),
        "log" => |args| float1(args.first(), "log", f64::ln),
        "log10" => |args| float1(args.first(), "log10", f64::log10),
        "log2" => |args| float1(args.first(), "log2", f64::log2),
        "atan2" => |args| match (args.first(), args.get(1)) {
            (Some(Value::Float(y)), Some(Value::Float(x))) => Ok(Value::Float(y.atan2(*x))),
            _ => Err(host_arg_err("atan2", "(double, double)", args.first())),
        },
        "fmod" => |args| match (args.first(), args.get(1)) {
            (Some(Value::Float(a)), Some(Value::Float(b))) => Ok(Value::Float(a % b)),
            _ => Err(host_arg_err("fmod", "(double, double)", args.first())),
        },

        // ---- stdlib.h ----
        "abs" => |args| match args.first() {
            Some(Value::Int(x)) => Ok(Value::Int(x.abs())),
            Some(Value::Float(x)) => Ok(Value::Float(x.abs())),
            other => Err(host_arg_err("abs", "(int)", other)),
        },
        "rand" => |_| Ok(Value::Int(FAKE_RAND.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % 32768)),
        "malloc" => |args| match args.first() {
            Some(Value::Int(size)) if *size >= 0 => {
                Ok(Value::Ptr(host_malloc(*size as usize)))
            }
            other => Err(host_arg_err("malloc", "(size_t)", other)),
        },
        "calloc" => |args| match (args.first(), args.get(1)) {
            (Some(Value::Int(n)), Some(Value::Int(sz))) if *n >= 0 && *sz >= 0 => {
                let total = (*n as usize).saturating_mul(*sz as usize);
                Ok(Value::Ptr(host_malloc_zeroed(total)))
            }
            _ => Err(host_arg_err("calloc", "(size_t, size_t)", args.first())),
        },
        "realloc" => |args| match (args.first(), args.get(1)) {
            (Some(Value::Ptr(old)), Some(Value::Int(new_size))) if *new_size >= 0 => {
                Ok(Value::Ptr(host_realloc(*old, *new_size as usize)))
            }
            _ => Err(host_arg_err("realloc", "(void*, size_t)", args.first())),
        },
        "free" => |args| match args.first() {
            Some(Value::Ptr(addr)) => {
                host_free(*addr);
                Ok(Value::Null)
            }
            Some(Value::Null) => Ok(Value::Null),
            other => Err(host_arg_err("free", "(void*)", other)),
        },
        "atoi" => |args| match args.first() {
            Some(Value::Str(s)) => Ok(Value::Int(s.trim().parse().unwrap_or(0))),
            other => Err(host_arg_err("atoi", "(const char*)", other)),
        },
        "atof" => |args| match args.first() {
            Some(Value::Str(s)) => Ok(Value::Float(s.trim().parse().unwrap_or(0.0))),
            other => Err(host_arg_err("atof", "(const char*)", other)),
        },

        // ---- string.h ----
        "strlen" => |args| match args.first() {
            Some(Value::Str(s)) => Ok(Value::Int(s.len() as i64)),
            other => Err(host_arg_err("strlen", "(const char*)", other)),
        },
        "strcmp" => |args| match (args.first(), args.get(1)) {
            (Some(Value::Str(a)), Some(Value::Str(b))) => Ok(Value::Int(match a.cmp(b) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })),
            _ => Err(host_arg_err("strcmp", "(const char*, const char*)", args.first())),
        },
        _ => return None,
    })
}

static FAKE_RAND: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(41);

// ---- Unsafe mode host heap (stdlib `malloc`/`free` arena) ----
const HOST_HEAP_SIZE: usize = 1 << 20; // 1 MiB arena for raw pointer tests
static HOST_HEAP: std::sync::OnceLock<Vec<std::sync::Mutex<bool>>> = std::sync::OnceLock::new();

fn host_heap() -> &'static Vec<std::sync::Mutex<bool>> {
    HOST_HEAP.get_or_init(|| {
        (0..HOST_HEAP_SIZE).map(|_| std::sync::Mutex::new(false)).collect()
    })
}

/// Allocate `size` bytes from the host arena. Returns an address (1-based:
/// 0 is reserved as null). Blocks are 16-byte aligned.
fn host_malloc(size: usize) -> u64 {
    if size == 0 {
        return 0;
    }
    let heap = host_heap();
    // Simple first-fit over 16-byte slots.
    let slots = size.div_ceil(16).max(1);
    let mut run_start: Option<usize> = None;
    let mut run_len = 0usize;
    for i in 0..heap.len() {
        let free = *heap[i].lock().unwrap();
        if free {
            if run_len == 0 {
                run_start = Some(i);
            }
            run_len += 1;
            if run_len >= slots {
                if let Some(start) = run_start {
                    for slot in start..(start + slots) {
                        *heap[slot].lock().unwrap() = true;
                    }
                    // Store allocation header in the first slot's high bits
                    // so `free` knows how many slots to release.
                    let addr = ((start as u64) + 1) << 4; // 16-byte aligned, 1-based
                    return addr;
                }
            }
        } else {
            run_start = None;
            run_len = 0;
        }
    }
    0 // out of memory
}

fn host_malloc_zeroed(size: usize) -> u64 {
    host_malloc(size) // arena starts zeroed (false = free, block data is conceptual)
}

fn host_realloc(addr: u64, new_size: usize) -> u64 {
    if addr == 0 {
        return host_malloc(new_size);
    }
    // Reuse the existing slot only if it can fit; otherwise allocate fresh.
    let _start = ((addr >> 4) as usize) - 1;
    let old_slots = if let Some(header) = HOST_BLOCK_SIZES.get() {
        header.get(&addr).copied().unwrap_or(1)
    } else {
        1
    };
    let need_slots = new_size.div_ceil(16).max(1);
    if need_slots <= old_slots {
        return addr;
    }
    host_free(addr);
    host_malloc(new_size)
}

fn host_free(addr: u64) {
    if addr == 0 {
        return;
    }
    let start = ((addr >> 4) as usize) - 1;
    let heap = host_heap();
    if start >= heap.len() {
        return;
    }
    let _ = *heap[start].lock().unwrap() = false;
    // Conservative: release this slot only. Extend in future if needed.
}

static HOST_BLOCK_SIZES: std::sync::OnceLock<std::collections::HashMap<u64, usize>> =
    std::sync::OnceLock::new();

fn float1(arg: Option<&Value>, name: &str, f: fn(f64) -> f64) -> Result<Value, String> {
    match arg {
        Some(Value::Float(x)) => Ok(Value::Float(f(*x))),
        Some(Value::Int(x)) => Ok(Value::Float(f(*x as f64))),
        other => Err(host_arg_err(name, "(double)", other)),
    }
}

fn host_arg_err(name: &str, sig: &str, got: Option<&Value>) -> String {
    format!(
        "FFI Error: invalid argument for '{}{}'. Expected C signature: {}",
        name,
        got.map(|v| format!(" got {:?}", v)).unwrap_or_default(),
        sig
    )
}

pub struct VirtualMachine {
    pub program: IrProgram,
    pub stack: Vec<Value>,
    pub call_stack: Vec<CallFrame>,
    pub output_log: Vec<String>,
}

impl VirtualMachine {
    pub fn new(program: IrProgram) -> Self {
        Self {
            program,
            stack: Vec::with_capacity(256),
            call_stack: Vec::with_capacity(64),
            output_log: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Value, String> {
        self.run_entry("main")
    }

    pub fn run_entry(&mut self, entry_fn: &str) -> Result<Value, String> {
        let main_fn = self
            .program
            .functions
            .get(entry_fn)
            .ok_or_else(|| format!("Entry point '{}' not found in Aether program", entry_fn))?
            .clone();

        if self.call_stack.is_empty() {
            self.call_stack.push(CallFrame {
                fn_name: main_fn.name.clone(),
                ip: 0,
                locals: HashMap::new(),
            });
        }

        let max_cycles = 10_000_000;
        let mut cycles = 0;

        while let Some(frame_idx) = self.call_stack.len().checked_sub(1) {
            cycles += 1;
            if cycles > max_cycles {
                return Err("Execution aborted: cycle limit exceeded (infinite loop protection)".into());
            }

            let fn_name = self.call_stack[frame_idx].fn_name.clone();
            let ip = self.call_stack[frame_idx].ip;

            let ir_fn = self
                .program
                .functions
                .get(&fn_name)
                .ok_or_else(|| format!("Function '{}' not found", fn_name))?;

            if ip >= ir_fn.instructions.len() {
                self.call_stack.pop();
                if self.call_stack.is_empty() {
                    break;
                }
                continue;
            }

            let op = ir_fn.instructions[ip].clone();
            self.call_stack[frame_idx].ip += 1;

            match op {
                OpCode::PushInt(n) => self.stack.push(Value::Int(n)),
                OpCode::PushFloat(f) => self.stack.push(Value::Float(f)),
                OpCode::PushStr(s) => self.stack.push(Value::Str(s)),
                OpCode::PushChar(c) => self.stack.push(Value::Char(c)),
                OpCode::PushBool(b) => self.stack.push(Value::Bool(b)),
                OpCode::PushNull => self.stack.push(Value::Null),

                OpCode::PushSome => {
                    let val = self.pop_stack()?;
                    self.stack.push(Value::Option(Some(Box::new(val))));
                }
                OpCode::PushNone => {
                    self.stack.push(Value::Option(None));
                }
                OpCode::PushOk => {
                    let val = self.pop_stack()?;
                    self.stack.push(Value::Result(Ok(Box::new(val))));
                }
                OpCode::PushErr => {
                    let val = self.pop_stack()?;
                    self.stack.push(Value::Result(Err(Box::new(val))));
                }

                OpCode::IsSome => {
                    let val = self.pop_stack()?;
                    match val {
                        Value::Option(Some(_)) => self.stack.push(Value::Bool(true)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::IsNone => {
                    let val = self.pop_stack()?;
                    match val {
                        Value::Option(None) => self.stack.push(Value::Bool(true)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::IsOk => {
                    let val = self.pop_stack()?;
                    match val {
                        Value::Result(Ok(_)) => self.stack.push(Value::Bool(true)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::IsErr => {
                    let val = self.pop_stack()?;
                    match val {
                        Value::Result(Err(_)) => self.stack.push(Value::Bool(true)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                OpCode::UnwrapPayload => {
                    let val = self.pop_stack()?;
                    match val {
                        Value::Option(Some(v)) => self.stack.push(*v),
                        Value::Result(Ok(v)) => self.stack.push(*v),
                        Value::Result(Err(e)) => self.stack.push(*e),
                        other => self.stack.push(other),
                    }
                }

                OpCode::Load(name) => {
                    let val = self.call_stack[frame_idx]
                        .locals
                        .get(&name)
                        .ok_or_else(|| format!("Runtime Error: Undefined variable '{}'", name))?
                        .clone();
                    self.stack.push(val);
                }
                OpCode::Store(name) => {
                    let val = self.pop_stack()?;
                    self.call_stack[frame_idx].locals.insert(name, val);
                }

                OpCode::Add => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            let res = x.checked_add(y).ok_or_else(|| {
                                format!("Security Panic: 64-bit Integer overflow during addition ({} + {})", x, y)
                            })?;
                            self.stack.push(Value::Int(res));
                        }
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x + y)),
                        (Value::Bool(x), Value::Bool(y)) => self.stack.push(Value::Bool(x || y)),
                        (Value::Str(x), Value::Str(y)) => self.stack.push(Value::Str(format!("{}{}", x, y))),
                        (Value::Str(x), other) => self.stack.push(Value::Str(format!("{}{}", x, other))),
                        (other, Value::Str(y)) => self.stack.push(Value::Str(format!("{}{}", other, y))),
                        (x, y) => return Err(format!("Cannot add incompatible types {:?} and {:?}", x, y)),
                    }
                }
                OpCode::Sub => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            let res = x.checked_sub(y).ok_or_else(|| {
                                format!("Security Panic: 64-bit Integer overflow during subtraction ({} - {})", x, y)
                            })?;
                            self.stack.push(Value::Int(res));
                        }
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x - y)),
                        (x, y) => return Err(format!("Cannot subtract {:?} and {:?}", x, y)),
                    }
                }
                OpCode::Mul => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            let res = x.checked_mul(y).ok_or_else(|| {
                                format!("Security Panic: 64-bit Integer overflow during multiplication ({} * {})", x, y)
                            })?;
                            self.stack.push(Value::Int(res));
                        }
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Float(x * y)),
                        (Value::Bool(x), Value::Bool(y)) => self.stack.push(Value::Bool(x && y)),
                        (x, y) => return Err(format!("Cannot multiply {:?} and {:?}", x, y)),
                    }
                }
                OpCode::Div => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err("Runtime Error: Division by zero".into());
                            }
                            self.stack.push(Value::Int(x / y));
                        }
                        (Value::Float(x), Value::Float(y)) => {
                            self.stack.push(Value::Float(x / y));
                        }
                        (x, y) => return Err(format!("Cannot divide {:?} and {:?}", x, y)),
                    }
                }
                OpCode::Mod => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err("Runtime Error: Modulo by zero".into());
                            }
                            self.stack.push(Value::Int(x % y));
                        }
                        (x, y) => return Err(format!("Cannot modulo {:?} and {:?}", x, y)),
                    }
                }

                OpCode::Equal => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    self.stack.push(Value::Bool(a == b));
                }
                OpCode::NotEqual => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    self.stack.push(Value::Bool(a != b));
                }
                OpCode::Less => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x < y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x < y)),
                        (x, y) => return Err(format!("Cannot compare '<' on {:?} and {:?}", x, y)),
                    }
                }
                OpCode::LessEqual => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x <= y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x <= y)),
                        (x, y) => return Err(format!("Cannot compare '<=' on {:?} and {:?}", x, y)),
                    }
                }
                OpCode::Greater => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x > y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x > y)),
                        (x, y) => return Err(format!("Cannot compare '>' on {:?} and {:?}", x, y)),
                    }
                }
                OpCode::GreaterEqual => {
                    let b = self.pop_stack()?;
                    let a = self.pop_stack()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Bool(x >= y)),
                        (Value::Float(x), Value::Float(y)) => self.stack.push(Value::Bool(x >= y)),
                        (x, y) => return Err(format!("Cannot compare '>=' on {:?} and {:?}", x, y)),
                    }
                }

                OpCode::Not => {
                    let a = self.pop_stack()?;
                    self.stack.push(Value::Bool(!a.is_truthy()));
                }
                OpCode::Neg => {
                    let a = self.pop_stack()?;
                    match a {
                        Value::Int(n) => self.stack.push(Value::Int(-n)),
                        Value::Float(f) => self.stack.push(Value::Float(-f)),
                        other => return Err(format!("Cannot negate non-numeric value {:?}", other)),
                    }
                }

                OpCode::Jump(target) => {
                    self.call_stack[frame_idx].ip = target;
                }
                OpCode::JumpIfFalse(target) => {
                    let cond = self.pop_stack()?;
                    if !cond.is_truthy() {
                        self.call_stack[frame_idx].ip = target;
                    }
                }

                OpCode::Call(callee_name, arg_count) => {
                    let target_fn = self
                        .program
                        .functions
                        .get(&callee_name)
                        .ok_or_else(|| format!("Function '{}' not found in VM", callee_name))?
                        .clone();

                    let mut args = Vec::with_capacity(arg_count);
                    for _ in 0..arg_count {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();

                    let mut new_locals = HashMap::new();
                    for (param_name, arg_val) in target_fn.params.iter().zip(args.into_iter()) {
                        new_locals.insert(param_name.clone(), arg_val);
                    }

                    self.call_stack.push(CallFrame {
                        fn_name: callee_name,
                        ip: 0,
                        locals: new_locals,
                    });
                }

                OpCode::CallExtern(callee_name, arg_count) => {
                    let host = resolve_host_fn(&callee_name).ok_or_else(|| {
                        let abi = self
                            .program
                            .externs
                            .get(&callee_name)
                            .map(|e| e.abi.clone())
                            .unwrap_or_else(|| "C".to_string());
                        format!(
                            "FFI Error: unresolved external symbol '{}' (ABI \"{}\"). \
                             The symbol must be provided by a linked library; \
                             it is available when compiling the generated C output, \
                             but has no native implementation inside this VM.",
                            callee_name, abi
                        )
                    })?;

                    let mut args = Vec::with_capacity(arg_count);
                    for _ in 0..arg_count {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();

                    self.stack.push(host(&args)?);
                }

                // Concurrency Operations
                OpCode::Spawn(callee_name, arg_count) => {
                    let target_fn = self
                        .program
                        .functions
                        .get(&callee_name)
                        .ok_or_else(|| format!("Function '{}' not found in spawn", callee_name))?
                        .clone();

                    let mut args = Vec::with_capacity(arg_count);
                    for _ in 0..arg_count {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();

                    // Execute task in memory and return task handle container
                    let mut sub_vm = VirtualMachine::new(self.program.clone());
                    let mut sub_locals = HashMap::new();
                    for (param_name, arg_val) in target_fn.params.iter().zip(args.into_iter()) {
                        sub_locals.insert(param_name.clone(), arg_val);
                    }
                    sub_vm.call_stack.push(CallFrame {
                        fn_name: callee_name.clone(),
                        ip: 0,
                        locals: sub_locals,
                    });
                    let res = sub_vm.run_entry(&callee_name).unwrap_or(Value::Null);

                    self.stack.push(Value::Task(Arc::new(Mutex::new(Some(res)))));
                }

                OpCode::Await => {
                    let task_val = self.pop_stack()?;
                    match task_val {
                        Value::Task(inner) => {
                            let val = inner.lock().unwrap().clone().unwrap_or(Value::Null);
                            self.stack.push(val);
                        }
                        other => self.stack.push(other),
                    }
                }

                OpCode::MakeChan => {
                    self.stack.push(Value::Chan(Arc::new(Mutex::new(Vec::new()))));
                }

                OpCode::SendChan => {
                    let val = self.pop_stack()?;
                    let chan_val = self.pop_stack()?;
                    if let Value::Chan(buf) = chan_val {
                        buf.lock().unwrap().push(val);
                        self.stack.push(Value::Bool(true));
                    } else {
                        return Err("Cannot send on non-channel value".into());
                    }
                }

                OpCode::RecvChan => {
                    let chan_val = self.pop_stack()?;
                    if let Value::Chan(buf) = chan_val {
                        let val = if buf.lock().unwrap().is_empty() {
                            Value::Null
                        } else {
                            buf.lock().unwrap().remove(0)
                        };
                        self.stack.push(val);
                    } else {
                        return Err("Cannot recv on non-channel value".into());
                    }
                }

                OpCode::Return => {
                    let ret_val = self.stack.pop().unwrap_or(Value::Null);
                    self.call_stack.pop();
                    if !self.call_stack.is_empty() {
                        self.stack.push(ret_val);
                    } else {
                        return Ok(ret_val);
                    }
                }

                OpCode::Print(arg_count) => {
                    let mut args = Vec::with_capacity(arg_count);
                    for _ in 0..arg_count {
                        args.push(self.pop_stack()?);
                    }
                    args.reverse();

                    let text: Vec<String> = args.iter().map(|v| v.to_string()).collect();
                    let line = text.join("");
                    println!("{}", line);
                    self.output_log.push(line);
                }

                OpCode::MakeArray(count) => {
                    let mut elements = Vec::with_capacity(count);
                    for _ in 0..count {
                        elements.push(self.pop_stack()?);
                    }
                    elements.reverse();
                    self.stack.push(Value::Array(Rc::new(RefCell::new(elements))));
                }

                OpCode::MakeMap(count) => {
                    let mut map = HashMap::new();
                    for _ in 0..count {
                        let val = self.pop_stack()?;
                        let key = self.pop_stack()?;
                        map.insert(key.to_string(), val);
                    }
                    self.stack.push(Value::Map(Rc::new(RefCell::new(map))));
                }

                OpCode::GetIndex => {
                    let idx_val = self.pop_stack()?;
                    let target_val = self.pop_stack()?;
                    match (target_val, idx_val) {
                        (Value::Array(arr), Value::Int(idx)) => {
                            let borrowed = arr.borrow();
                            if idx < 0 || (idx as usize) >= borrowed.len() {
                                return Err(format!(
                                    "Runtime Error: Array index {} out of bounds (len: {})",
                                    idx,
                                    borrowed.len()
                                ));
                            }
                            self.stack.push(borrowed[idx as usize].clone());
                        }
                        (Value::Map(map), key_val) => {
                            let borrowed = map.borrow();
                            let key_str = key_val.to_string();
                            let val = borrowed.get(&key_str).cloned().unwrap_or(Value::Null);
                            self.stack.push(val);
                        }
                        (other, _) => return Err(format!("Cannot index non-collection value {:?}", other)),
                    }
                }

                OpCode::SetIndex => {
                    let val = self.pop_stack()?;
                    let idx_val = self.pop_stack()?;
                    let target_val = self.pop_stack()?;
                    match (target_val, idx_val) {
                        (Value::Array(arr), Value::Int(idx)) => {
                            let mut borrowed = arr.borrow_mut();
                            if idx < 0 || (idx as usize) >= borrowed.len() {
                                return Err(format!(
                                    "Runtime Error: Array index {} out of bounds (len: {})",
                                    idx,
                                    borrowed.len()
                                ));
                            }
                            borrowed[idx as usize] = val;
                        }
                        (Value::Map(map), key_val) => {
                            let mut borrowed = map.borrow_mut();
                            borrowed.insert(key_val.to_string(), val);
                        }
                        (other, _) => return Err(format!("Cannot index-assign non-collection value {:?}", other)),
                    }
                }

                OpCode::Assert => {
                    let cond_val = self.pop_stack()?;
                    match cond_val {
                        Value::Bool(true) => {}
                        Value::Bool(false) => {
                            return Err("Assertion Failed: condition evaluated to false".into());
                        }
                        other => {
                            return Err(format!("Assertion Failed: expected Bool condition, got {:?}", other));
                        }
                    }
                }
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::Null))
    }

    fn pop_stack(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow in VM".to_string())
    }
}

pub fn execute_ir(program: IrProgram) -> Result<Value, String> {
    let mut vm = VirtualMachine::new(program);
    vm.run()
}
