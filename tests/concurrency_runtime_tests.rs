use sorayunara::concurrency_runtime::{
    Actor, AtomicInt, Channel, Mutex, RwLock, Semaphore, TaskScope,
};
use std::sync::Arc;
use std::thread;

#[test]
fn test_concurrency_channel_mpsc() {
    let chan = Arc::new(Channel::new(Some(10)));
    let chan_clone = chan.clone();

    let handle = thread::spawn(move || {
        for i in 1..=5 {
            chan_clone.send(i * 10).unwrap();
        }
    });

    handle.join().unwrap();

    let mut received = Vec::new();
    for _ in 1..=5 {
        received.push(chan.recv().unwrap());
    }
    assert_eq!(received, vec![10, 20, 30, 40, 50]);
}

#[test]
fn test_concurrency_mutex_and_rwlock() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let m_clone = m.clone();
        handles.push(thread::spawn(move || {
            let mut guard = m_clone.lock();
            *guard += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(*m.lock(), 10);

    let rw = RwLock::new("initial".to_string());
    {
        let r1 = rw.read();
        let r2 = rw.read();
        assert_eq!(*r1, "initial");
        assert_eq!(*r2, "initial");
    }
    {
        let mut w = rw.write();
        *w = "updated".to_string();
    }
    assert_eq!(*rw.read(), "updated");
}

#[test]
fn test_concurrency_semaphore_and_atomic() {
    let sem = Semaphore::new(2);
    sem.acquire();
    sem.acquire();
    sem.release();
    sem.acquire();

    let atomic = AtomicInt::new(100);
    assert_eq!(atomic.fetch_add(50), 100);
    assert_eq!(atomic.load(), 150);
    assert_eq!(atomic.compare_exchange(150, 200), Ok(150));
    assert_eq!(atomic.load(), 200);
}

#[test]
fn test_concurrency_actor_and_structured_scope() {
    let actor = Actor::spawn(String::new(), |st: &mut String, msg: String| {
        *st = msg;
    });
    actor.send("PING".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let msg = actor.get_state(|s| s.clone());
    assert_eq!(msg, "PING");

    let scope = TaskScope::new("batch_processing");
    let flag = Arc::new(AtomicInt::new(0));

    let flag_clone = flag.clone();
    scope.spawn_scoped(move || {
        flag_clone.store(42);
    });

    scope.join_all();
    assert_eq!(flag.load(), 42);
}
