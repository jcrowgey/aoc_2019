use std::env::args;
use std::io;

mod one;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() > 1 {
        let stdin = io::stdin();
        let buf = stdin.lock();
        match args[1].as_ref() {
            "1a" => println!("{}", one::one_a(buf)),
            "1b" => println!("{}", one::one_b(buf)),
            _ => println!("argument unrecognized: {}", args[1]),
        }
    }
}
