#[path = "intcode.rs"]
mod intcode;

use std::io::BufRead;
use std::thread;
use intcode::IntcodeMachine;

pub fn five_a<I>(buf: I) -> i64
where
    I: BufRead,
{
    let p = intcode::read_program(buf);
    let (mach_in, mach_out, mut mach) = IntcodeMachine::new(p);

    mach_in.send(1).unwrap();
    let _exit_code = thread::spawn(move || {
        mach.run_program()
    });

    let mut out_vec = Vec::new();
    for i in mach_out {
        out_vec.push(i);
    }
    out_vec[out_vec.len()-1]
}


pub fn five_b<I>(buf: I) -> i64
where
    I: BufRead,
{
    let p = intcode::read_program(buf);
    let (mach_in, mach_out, mut mach) = IntcodeMachine::new(p);
    mach_in.send(5).unwrap();
    let _exit_code = thread::spawn(move || {
        mach.run_program()
    });

    let mut out_vec = Vec::new();
    for i in mach_out {
        out_vec.push(i);
    }
    out_vec[out_vec.len()-1]
}
