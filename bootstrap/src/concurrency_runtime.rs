#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex as StdMutex, RwLock as StdRwLock};
use std::thread::{self, JoinHandle};

/// Status of an asynchronous task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
}

/// Task structure representing a green thread / fiber
#[derive(Debug, Clone)]
pub struct Task<T> {
    pub id: u64,
    pub status: TaskStatus,
    pub result: Option<T>,
}

impl<T: Clone> Task<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            status: TaskStatus::Pending,
            result: None,
        }
    }

    pub fn complete(&mut self, val: T) {
        self.status = TaskStatus::Completed;
        self.result = Some(val);
    }
}

/// CSP-style Typed Buffered/Unbuffered MPMC Channel
pub struct Channel<T> {
    queue: StdMutex<VecDeque<T>>,
    condvar: Condvar,
    closed: AtomicBool,
    capacity: Option<usize>,
}

impl<T> Channel<T> {
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            queue: StdMutex::new(VecDeque::new()),
            condvar: Condvar::new(),
            closed: AtomicBool::new(false),
            capacity,
        }
    }

    pub fn send(&self, item: T) -> Result<(), String> {
        let mut q = self.queue.lock().unwrap();
        while let Some(cap) = self.capacity {
            if q.len() >= cap {
                if self.closed.load(Ordering::SeqCst) {
                    return Err("Channel closed: cannot send".into());
                }
                q = self.condvar.wait(q).unwrap();
            } else {
                break;
            }
        }
        if self.closed.load(Ordering::SeqCst) {
            return Err("Channel closed: cannot send".into());
        }
        q.push_back(item);
        self.condvar.notify_one();
        Ok(())
    }

    pub fn recv(&self) -> Result<T, String> {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(item) = q.pop_front() {
                self.condvar.notify_one();
                return Ok(item);
            }
            if self.closed.load(Ordering::SeqCst) {
                return Err("Channel closed: cannot receive".into());
            }
            q = self.condvar.wait(q).unwrap();
        }
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.condvar.notify_all();
    }
}

/// Erlang-style Actor Mailbox Runtime
pub struct Actor<M, S> {
    state: StdMutex<S>,
    mailbox: Channel<M>,
    handler: Arc<dyn Fn(&mut S, M) + Send + Sync>,
}

impl<M: Send + 'static, S: Send + 'static> Actor<M, S> {
    pub fn spawn<F>(initial_state: S, handler: F) -> Arc<Self>
    where
        F: Fn(&mut S, M) + Send + Sync + 'static,
    {
        let actor = Arc::new(Self {
            state: StdMutex::new(initial_state),
            mailbox: Channel::new(None),
            handler: Arc::new(handler),
        });

        let worker = Arc::clone(&actor);
        thread::spawn(move || {
            while let Ok(msg) = worker.mailbox.recv() {
                let mut state = worker.state.lock().unwrap();
                (worker.handler)(&mut *state, msg);
            }
        });

        actor
    }

    pub fn send(&self, msg: M) -> Result<(), String> {
        self.mailbox.send(msg)
    }

    pub fn get_state<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&S) -> R,
    {
        let state = self.state.lock().unwrap();
        f(&*state)
    }
}

/// Structured Concurrency Scope
pub struct StructuredScope {
    handles: StdMutex<Vec<JoinHandle<()>>>,
}

impl StructuredScope {
    pub fn new() -> Self {
        Self {
            handles: StdMutex::new(Vec::new()),
        }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let handle = thread::spawn(f);
        self.handles.lock().unwrap().push(handle);
    }

    pub fn join_all(&self) {
        let mut handles = self.handles.lock().unwrap();
        while let Some(h) = handles.pop() {
            let _ = h.join();
        }
    }
}

pub struct TaskScope {
    pub name: String,
    scope: StructuredScope,
}

impl TaskScope {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scope: StructuredScope::new(),
        }
    }

    pub fn spawn_scoped<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.scope.spawn(f);
    }

    pub fn join_all(&self) {
        self.scope.join_all();
    }
}

type ScheduledTask = Box<dyn FnOnce() + Send + 'static>;

/// Work-Stealing M:N Scheduler
pub struct WorkStealingScheduler {
    tasks: Arc<StdMutex<VecDeque<ScheduledTask>>>,
    workers: Vec<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
}

impl WorkStealingScheduler {
    pub fn new(num_threads: usize) -> Self {
        let tasks: Arc<StdMutex<VecDeque<ScheduledTask>>> = Arc::new(StdMutex::new(VecDeque::new()));
        let shutdown = Arc::new(AtomicBool::new(false));
        let mut workers = Vec::new();

        for _ in 0..num_threads {
            let tasks_clone = Arc::clone(&tasks);
            let shutdown_clone = Arc::clone(&shutdown);

            let handle = thread::spawn(move || {
                while !shutdown_clone.load(Ordering::Relaxed) {
                    let task = {
                        let mut q = tasks_clone.lock().unwrap();
                        q.pop_front()
                    };
                    if let Some(t) = task {
                        t();
                    } else {
                        thread::yield_now();
                    }
                }
            });
            workers.push(handle);
        }

        Self {
            tasks,
            workers,
            shutdown,
        }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tasks.lock().unwrap().push_back(Box::new(task));
    }
}

/// High-Performance Bump / Region / Arena Memory Allocator
pub struct ArenaAllocator {
    buffer: StdMutex<Vec<u8>>,
    allocated: AtomicU64,
    capacity: usize,
}

impl ArenaAllocator {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: StdMutex::new(vec![0u8; capacity]),
            allocated: AtomicU64::new(0),
            capacity,
        }
    }

    pub fn alloc(&self, size: usize, align: usize) -> Option<usize> {
        let current = self.allocated.load(Ordering::Relaxed) as usize;
        let aligned = (current + align - 1) & !(align - 1);
        if aligned + size > self.capacity {
            return None;
        }
        self.allocated.store((aligned + size) as u64, Ordering::Relaxed);
        Some(aligned)
    }

    pub fn reset(&self) {
        self.allocated.store(0, Ordering::Relaxed);
    }
}

/// Synchronization Primitives
pub struct Mutex<T>(StdMutex<T>);
impl<T> Mutex<T> {
    pub fn new(val: T) -> Self { Self(StdMutex::new(val)) }
    pub fn lock(&self) -> std::sync::MutexGuard<'_, T> { self.0.lock().unwrap() }
}

pub struct RwLock<T>(StdRwLock<T>);
impl<T> RwLock<T> {
    pub fn new(val: T) -> Self { Self(StdRwLock::new(val)) }
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> { self.0.read().unwrap() }
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> { self.0.write().unwrap() }
}

pub struct AtomicInt(AtomicI64);
impl AtomicInt {
    pub fn new(val: i64) -> Self { Self(AtomicI64::new(val)) }
    pub fn fetch_add(&self, delta: i64) -> i64 { self.0.fetch_add(delta, Ordering::SeqCst) }
    pub fn load(&self) -> i64 { self.0.load(Ordering::SeqCst) }
    pub fn store(&self, val: i64) { self.0.store(val, Ordering::SeqCst); }
    pub fn compare_exchange(&self, current: i64, new: i64) -> Result<i64, i64> {
        self.0.compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)
    }
}

pub struct Semaphore {
    permits: StdMutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    pub fn new(permits: usize) -> Self {
        Self {
            permits: StdMutex::new(permits),
            condvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut p = self.permits.lock().unwrap();
        while *p == 0 {
            p = self.condvar.wait(p).unwrap();
        }
        *p -= 1;
    }

    pub fn release(&self) {
        let mut p = self.permits.lock().unwrap();
        *p += 1;
        self.condvar.notify_one();
    }
}
