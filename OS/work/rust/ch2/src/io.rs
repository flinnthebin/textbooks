use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

fn main() {
    let path = "/tmp/file";
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            exit(1);
        }
    };

    let contents = "Hello World!\n";
    if let Err(e) = file.write_all(contents.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
        exit(1);
    }
}
