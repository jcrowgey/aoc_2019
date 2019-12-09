use std::env::args;
use std::io;

mod one;
mod two;
mod three;
mod four;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() > 1 {
        let stdin = io::stdin();
        let buf = stdin.lock();
        match args[1].as_ref() {
            "1a" => println!("{}", one::one_a(buf)),
            "1b" => println!("{}", one::one_b(buf)),
            "2a" => println!("{}", two::two_a(buf)),
            "2b" => println!("{:?}", two::two_b(buf)),
            "3a" => println!("{}", three::three_a(buf)),
            "3b" => println!("{}", three::three_b(buf)),
            "4a" => println!("{}", four::four_a(buf)),
            "4b" => println!("{}", four::four_b(buf)),
            _ => println!("argument unrecognized: {}", args[1]),
        }
    }
}
