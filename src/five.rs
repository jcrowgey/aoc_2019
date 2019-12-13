#[path = "intcode.rs"]
mod intcode;

use std::io::BufRead;
use intcode::IntcodeMachine;

pub fn five_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let p = intcode::read_program(buf);
    let mut mach = IntcodeMachine::new(p);
    mach.input(1);
    let _ret = mach.run_program();
    let mut out_vec = Vec::new();

    while let Some(i) = mach.output() {
        out_vec.push(i);
    }
    out_vec[out_vec.len()-1]
}
