use std::env;

mod common;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cpu <string>");
        std::process::exit(1);
    }
    
    let input = & args[1];
    loop {
        common::spin(1);
        println!("{}", input);
    }
}
