#[path = "intcode.rs"]
mod intcode;

use std::io::BufRead;
use std::thread;
use intcode::IntcodeMachine;


pub fn nine_a<I>(buf: I) -> i64
where
    I: BufRead,
{

    let p = intcode::read_program(buf);
    let (m_in, m_out, mut mach) = IntcodeMachine::new(p);

    m_in.send(1).unwrap();
    thread::spawn(move || {
        mach.run_program();
    });
    m_out.recv().unwrap()
}

pub fn nine_b<I>(buf: I) -> Vec<i64>
where
    I: BufRead,
{

    let p = intcode::read_program(buf);
    let (m_in, m_out, mut mach) = IntcodeMachine::new(p);

    m_in.send(2).unwrap();
    thread::spawn(move || {
        mach.run_program();
    });
    let mut res = Vec::new();
    for o in m_out {
        res.push(o);
    }
    res.reverse();
    res
}
