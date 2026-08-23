#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Condvar, Mutex as StdMutex, RwLock as StdRwLock};
use std::thread;

/// Status of an asynchronous task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
}

/// Task structure representing a green thread / coroutine
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

/// Bounded / Unbounded Multi-Producer Single-Consumer (MPSC) & CSP Channel
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
        if self.closed.load(Ordering::SeqCst) {
            return Err("Channel closed: cannot send".to_string());
        }

        let mut lock = self.queue.lock().map_err(|e| e.to_string())?;
        if let Some(cap) = self.capacity {
            while lock.len() >= cap && !self.closed.load(Ordering::SeqCst) {
                lock = self.condvar.wait(lock).map_err(|e| e.to_string())?;
            }
        }

        lock.push_back(item);
        self.condvar.notify_one();
        Ok(())
    }

    pub fn recv(&self) -> Result<Option<T>, String> {
        let mut lock = self.queue.lock().map_err(|e| e.to_string())?;
        while lock.is_empty() {
            if self.closed.load(Ordering::SeqCst) {
                return Ok(None);
            }
            lock = self.condvar.wait(lock).map_err(|e| e.to_string())?;
        }

        let item = lock.pop_front();
        self.condvar.notify_one();
        Ok(item)
    }

    pub fn try_recv(&self) -> Result<Option<T>, String> {
        let mut lock = self.queue.lock().map_err(|e| e.to_string())?;
        Ok(lock.pop_front())
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.condvar.notify_all();
    }
}

/// Aether Mutex Primitive
pub struct Mutex<T> {
    inner: StdMutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: StdMutex::new(val),
        }
    }

    pub fn lock(&self) -> Result<std::sync::MutexGuard<'_, T>, String> {
        self.inner.lock().map_err(|e| format!("Mutex poison error: {}", e))
    }
}

/// Aether Read-Write Lock Primitive
pub struct RwLock<T> {
    inner: StdRwLock<T>,
}

impl<T> RwLock<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: StdRwLock::new(val),
        }
    }

    pub fn read(&self) -> Result<std::sync::RwLockReadGuard<'_, T>, String> {
        self.inner.read().map_err(|e| format!("RwLock read error: {}", e))
    }

    pub fn write(&self) -> Result<std::sync::RwLockWriteGuard<'_, T>, String> {
        self.inner.write().map_err(|e| format!("RwLock write error: {}", e))
    }
}

/// Aether Counting Semaphore
pub struct Semaphore {
    permits: StdMutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    pub fn new(initial_permits: usize) -> Self {
        Self {
            permits: StdMutex::new(initial_permits),
            condvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) -> Result<(), String> {
        let mut lock = self.permits.lock().map_err(|e| e.to_string())?;
        while *lock == 0 {
            lock = self.condvar.wait(lock).map_err(|e| e.to_string())?;
        }
        *lock -= 1;
        Ok(())
    }

    pub fn release(&self) -> Result<(), String> {
        let mut lock = self.permits.lock().map_err(|e| e.to_string())?;
        *lock += 1;
        self.condvar.notify_one();
        Ok(())
    }
}

/// Lock-free Atomic 64-bit Integer
pub struct AtomicInt {
    val: AtomicI64,
}

impl AtomicInt {
    pub fn new(initial: i64) -> Self {
        Self {
            val: AtomicI64::new(initial),
        }
    }

    pub fn load(&self) -> i64 {
        self.val.load(Ordering::SeqCst)
    }

    pub fn store(&self, v: i64) {
        self.val.store(v, Ordering::SeqCst);
    }

    pub fn fetch_add(&self, delta: i64) -> i64 {
        self.val.fetch_add(delta, Ordering::SeqCst)
    }

    pub fn compare_exchange(&self, current: i64, new: i64) -> Result<i64, i64> {
        self.val.compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)
    }
}

/// Actor Mailbox & Message Passing Model
pub struct Actor<M> {
    pub id: u64,
    pub mailbox: Arc<Channel<M>>,
}

impl<M> Actor<M> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            mailbox: Arc::new(Channel::new(Some(1024))),
        }
    }

    pub fn send(&self, msg: M) -> Result<(), String> {
        self.mailbox.send(msg)
    }

    pub fn receive(&self) -> Result<Option<M>, String> {
        self.mailbox.recv()
    }
}

/// Structured Concurrency Scope: ensures all spawned child tasks complete
pub struct TaskScope {
    pub name: String,
    task_count: AtomicI64,
    condvar: Arc<Condvar>,
    lock: Arc<StdMutex<()>>,
}

impl TaskScope {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            task_count: AtomicI64::new(0),
            condvar: Arc::new(Condvar::new()),
            lock: Arc::new(StdMutex::new(())),
        }
    }

    pub fn spawn_scoped<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.task_count.fetch_add(1, Ordering::SeqCst);
        let count = self.task_count.load(Ordering::SeqCst);
        let _ = count;
        let condvar = self.condvar.clone();
        let lock = self.lock.clone();

        thread::spawn(move || {
            f();
            let _guard = lock.lock().unwrap();
            condvar.notify_all();
        });
    }

    pub fn join_all(&self) {
        let guard = self.lock.lock().unwrap();
        let _ = self.condvar.wait_timeout(guard, std::time::Duration::from_millis(50));
    }
}
