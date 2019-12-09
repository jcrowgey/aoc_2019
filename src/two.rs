#[path = "intcode.rs"]
mod intcode;

use std::io::BufRead;
use intcode::IntcodeMachine;


fn read_program<I>(mut buf: I) -> Vec<usize>
where
    I: BufRead,
{
    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    line.trim()
        .split(",")
        .into_iter()
        .map(|x| x.parse().expect("error parsing number"))
        .collect()
}

pub fn two_a<I>(buf: I) -> usize
where
    I: BufRead,
{

    let mut p = read_program(buf);
    p[1] = 12;
    p[2] = 2;

    let mut mach = IntcodeMachine::new(p);
    mach.run_program()
}

pub fn two_b<I>(buf: I) -> usize
where
    I: BufRead,
{

    let p = read_program(buf);
    let mut noun: usize = 0;
    let mut out: usize;
    let needle: usize = 19690720;
    while noun < 100 {
        let mut verb: usize = 0;
        while verb < 100 {
            let mut q = p.to_owned();
            q[1] = noun;
            q[2] = verb;
            let mut mach = IntcodeMachine::new(q);
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
