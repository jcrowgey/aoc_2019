use std::env::args;
use std::io;

mod intcode;
mod point;

mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;

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
            "5a" => println!("{}", five::five_a(buf)),
            "5b" => println!("{}", five::five_b(buf)),
            "6a" => println!("{}", six::six_a(buf)),
            "6b" => println!("{}", six::six_b(buf)),
            "7a" => println!("{}", seven::seven_a(buf)),
            "7b" => println!("{}", seven::seven_b(buf)),
            "8a" => println!("{}", eight::eight_a(buf)),
            "8b" => {
                eight::eight_b(buf);
                println!("\n[see above]")
            }
            "9a" => println!("{}", nine::nine_a(buf)),
            "9b" => println!("{:?}", nine::nine_b(buf)),
            "10a" => println!("{}", ten::ten_a(buf)),
            "10b" => println!("{}", ten::ten_b(buf)),
            "11a" => println!("{}", eleven::eleven_a(buf)),
            "11b" => println!("{}", eleven::eleven_b(buf)),
            "12a" => println!("{}", twelve::twelve_a(buf)),
            "12b" => println!("{}", twelve::twelve_b(buf)),
            "13a" => println!("{}", thirteen::thirteen_a(buf)),
            "13b" => println!("{}", thirteen::thirteen_b(buf)),
            "13p" => thirteen::play_interactive(buf),
            "14a" => println!("{}", fourteen::fourteen_a(buf)),
            "14b" => println!("{}", fourteen::fourteen_b(buf)),
            "15a" => println!("{}", fifteen::fifteen_a(buf)),
            "15b" => println!("{}", fifteen::fifteen_b(buf)),
            _ => println!("argument unrecognized: {}", args[1]),
        }
    }
}
