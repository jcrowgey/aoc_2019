use std::io::BufRead;
use crate::intcode;
use crate::intcode::IntcodeMachine;


pub fn two_a<I>(buf: I) -> i64
where
    I: BufRead,
{

    let mut p = intcode::read_program(buf);
    p[1] = 12;
    p[2] = 2;

    let (_m_in, _m_out, mut mach) = IntcodeMachine::new(p);
    mach.run_program()
}

pub fn two_b<I>(buf: I) -> i64
where
    I: BufRead,
{

    let p = intcode::read_program(buf);
    let mut noun: i64 = 0;
    let mut out: i64;
    let needle: i64 = 19690720;
    while noun < 100 {
        let mut verb: i64 = 0;
        while verb < 100 {
            let mut q = p.to_owned();
            q[1] = noun;
            q[2] = verb;
            let (_m_in, _m_out, mut mach) = IntcodeMachine::new(q);
            out = mach.run_program();

            if needle == out {
                return (100 * noun) + verb;
            }
            verb += 1;
        }
        noun += 1;
    }

    panic!("no input yields value 19690720");
}
