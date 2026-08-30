use sorayunara::concurrency_runtime::{Actor, ArenaAllocator, Channel, Semaphore};
use std::sync::Arc;
use std::time::Instant;

fn bench_channel_throughput() {
    let chan = Arc::new(Channel::new(Some(1000)));
    let chan_tx = Arc::clone(&chan);
    let count = 100_000;

    let start = Instant::now();
    let producer = std::thread::spawn(move || {
        for i in 0..count {
            let _ = chan_tx.send(i);
        }
    });

    let mut received = 0;
    for _ in 0..count {
        if chan.recv().is_ok() {
            received += 1;
        }
    }
    producer.join().unwrap();
    let elapsed = start.elapsed();
    println!("Channel MPMC Throughput: {} msgs in {:.2?} ({:.2} M/s)", count, elapsed, (count as f64 / elapsed.as_secs_f64()) / 1_000_000.0);
    assert_eq!(received, count);
}

fn bench_actor_message_dispatch() {
    let count = 50_000;
    let actor = Actor::spawn(0i64, |state, msg: i64| {
        *state += msg;
    });

    let start = Instant::now();
    for _ in 0..count {
        let _ = actor.send(1);
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
    let elapsed = start.elapsed();
    let final_val = actor.get_state(|s| *s);
    println!("Actor Dispatch: {} messages processed in {:.2?} (Final count: {})", count, elapsed, final_val);
}

fn bench_arena_allocator() {
    let arena = ArenaAllocator::new(10 * 1024 * 1024); // 10MB
    let count = 200_000;

    let start = Instant::now();
    for _ in 0..count {
        let _ = arena.alloc(32, 8);
    }
    let elapsed = start.elapsed();
    println!("Arena Allocation: {} allocs in {:.2?} ({:.2} ns/alloc)", count, elapsed, (elapsed.as_nanos() as f64) / count as f64);
}

#[test]
fn test_benchmark_runtime_concurrency_suite() {
    bench_channel_throughput();
    bench_actor_message_dispatch();
    bench_arena_allocator();
}
