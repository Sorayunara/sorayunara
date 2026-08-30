use sorayunara::concurrency_runtime::{
    Actor, ArenaAllocator, AtomicInt, Channel, Mutex, RwLock, Semaphore, StructuredScope,
};
use std::sync::Arc;

#[test]
fn test_runtime_channel_bounded_and_closed() {
    let chan = Arc::new(Channel::new(Some(2)));
    let tx = Arc::clone(&chan);

    let h = std::thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.close();
    });

    assert_eq!(chan.recv().unwrap(), 1);
    assert_eq!(chan.recv().unwrap(), 2);
    assert!(chan.recv().is_err());
    h.join().unwrap();
}

#[test]
fn test_runtime_actor_state_mutation() {
    let actor = Actor::spawn(100, |state, delta: i32| {
        *state += delta;
    });

    actor.send(50).unwrap();
    actor.send(-20).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(20));
    assert_eq!(actor.get_state(|s| *s), 130);
}

#[test]
fn test_runtime_arena_allocator_alignment_and_reset() {
    let arena = ArenaAllocator::new(1024);
    let off1 = arena.alloc(10, 8).unwrap();
    let off2 = arena.alloc(20, 8).unwrap();

    assert_eq!(off1 % 8, 0);
    assert_eq!(off2 % 8, 0);
    assert!(off2 >= off1 + 10);

    arena.reset();
    let off3 = arena.alloc(10, 8).unwrap();
    assert_eq!(off3, 0);
}

#[test]
fn test_runtime_synchronization_primitives() {
    let mutex = Mutex::new(0);
    {
        let mut g = mutex.lock();
        *g = 42;
    }
    assert_eq!(*mutex.lock(), 42);

    let rwlock = RwLock::new(10);
    assert_eq!(*rwlock.read(), 10);

    let atomic = AtomicInt::new(0);
    assert_eq!(atomic.fetch_add(5), 0);
    assert_eq!(atomic.load(), 5);

    let sem = Semaphore::new(1);
    sem.acquire();
    sem.release();
}

#[test]
fn test_runtime_structured_concurrency_scope() {
    let scope = StructuredScope::new();
    let counter = Arc::new(AtomicInt::new(0));

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        scope.spawn(move || {
            c.fetch_add(1);
        });
    }

    scope.join_all();
    assert_eq!(counter.load(), 5);
}
