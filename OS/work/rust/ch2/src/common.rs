use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn get_time() -> f64 {
    let start = SystemTime::now();
    let duration = start
        .duration_since(UNIX_EPOCH)
        .expect("Timing Error");
    duration.as_secs() as f64 + duration.subsec_micros() as f64 / 1e6
}

pub fn spin(howlong: u64) {
    let start = Instant::now();
    while (Instant::now() - start) < Duration::from_secs(howlong) {
        // do nothing in loop
    }
}
