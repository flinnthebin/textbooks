use std::sync::atomic::{AtomicI32, Ordering};
use tokio::task;

static COUNTER: AtomicI32 = AtomicI32::new(0);

async fn worker(loops: i32) {
    for _ in 0..loops {
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: threads <value>");
        std::process::exit(1);
    }

    let loops: i32 = args[1]
        .parse()
        .expect("Invalid number");

    println!("Initial value: {}", COUNTER.load(Ordering::SeqCst));

    let handle_one = task::spawn(worker(loops));
    let handle_two = task::spawn(worker(loops));

    let (res_one, res_two) = tokio::join!(handle_one, handle_two);

    if let Err(e) = res_one {
        eprintln!("Error (Task 1): {:?}", e);
    }

    if let Err(e) = res_two {
        eprintln!("Error (Task 2): {:?}", e);
    }

    println!("Final value: {}", COUNTER.load(Ordering::SeqCst));
}
