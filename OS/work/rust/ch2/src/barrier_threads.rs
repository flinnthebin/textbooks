use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::env;
use std::process;
use tokio::sync::Barrier;
use tokio::task;

static COUNTER: AtomicI32 = AtomicI32::new(0);

async fn worker(loops: i32, barrier: Arc<Barrier>) {
    for _ in 0..loops {
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }
    barrier.wait().await;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: threads <value>");
        process::exit(1);
    }

    let loops: i32 = args[1]
        .parse()
        .expect("Invalid number");

    println!("Initial value: {}", COUNTER.load(Ordering::SeqCst));

    let barrier = Arc::new(Barrier::new(3));

    let barrier_one = Arc::clone(&barrier);
    let barrier_two = Arc::clone(&barrier);

    let handle_one = task::spawn(worker(loops, barrier_one));
    let handle_two = task::spawn(worker(loops, barrier_two));

    barrier.wait().await;

    handle_one.await
        .expect("Thread one panicked");
    handle_two.await
        .expect("Thread two panicked");

    println!("Final value: {}", COUNTER.load(Ordering::SeqCst));
}
