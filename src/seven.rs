#[path = "intcode.rs"]
mod intcode;

use std::io::BufRead;
use intcode::IntcodeMachine;

fn amp_chain(
    program: Vec<i32>,
    a_phase: i32,
    b_phase: i32,
    c_phase: i32,
    d_phase: i32,
    e_phase: i32
) -> i32 {
    let (a_in, a_out, mut amp_a) = IntcodeMachine::new(program.to_owned());
    let (b_in, b_out, mut amp_b) = IntcodeMachine::new(program.to_owned());
    let (c_in, c_out, mut amp_c) = IntcodeMachine::new(program.to_owned());
    let (d_in, d_out, mut amp_d) = IntcodeMachine::new(program.to_owned());
    let (e_in, e_out, mut amp_e) = IntcodeMachine::new(program);

    a_in.send(a_phase).unwrap();
    b_in.send(b_phase).unwrap();
    c_in.send(c_phase).unwrap();
    d_in.send(d_phase).unwrap();
    e_in.send(e_phase).unwrap();

    a_in.send(0).unwrap();
    let mut _exit_code = amp_a.run_program();

    b_in.send(a_out.recv().unwrap()).unwrap();
    _exit_code = amp_b.run_program();

    c_in.send(b_out.recv().unwrap()).unwrap();
    _exit_code = amp_c.run_program();

    d_in.send(c_out.recv().unwrap()).unwrap();
    _exit_code = amp_d.run_program();

    e_in.send(d_out.recv().unwrap()).unwrap();
    _exit_code = amp_e.run_program();

    e_out.recv().unwrap()
}

fn heaps(n: usize, a: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    if n == 1 {
        ret.push(a.to_vec());
        return ret
    } else {
        for i in 0 .. n-1 {
            ret.append(&mut heaps(n-1, a));

            if n % 2 == 0 {
                a.swap(i, n-1);
            } else {
                a.swap(0, n-1);
            }
        }
        ret.append(&mut heaps(n-1, a));
    }
    ret
}

pub fn seven_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let p = intcode::read_program(buf);
    let mut phases = vec![0,1,2,3,4];
    let mut o = 0;
    let mut max = o;
    for v in heaps(5, &mut phases).iter() {
        o = amp_chain(p.to_owned(),v[0],v[1],v[2],v[3],v[4]);
        if o > max {
            max = o;
        }
    }
    max
}


/*
pub fn five_b<I>(buf: I) -> i32
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
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_a() {
        let program = b"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(seven_a(&program [..]), 43210);

        let program = b"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(seven_a(&program [..]), 54321);

        let program = b"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(seven_a(&program [..]), 65210);

    }
}
