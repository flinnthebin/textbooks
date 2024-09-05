use std::process;

mod common;

fn main() {
    let p: Box<i32> = Box::new(0);
    println!("({}) address pointed to by p: {:p}", process::id(), &*p);

    let mut p = p;
    loop {
        common::spin(1);
        *p += 1;
        println!("({}) p: {}", process::id(), *p);
    }
}
