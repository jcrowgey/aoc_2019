#[path = "intcode.rs"]
mod intcode;

use std::io::BufRead;
use std::thread;
use intcode::IntcodeMachine;

fn amp_chain_feedback(
    program: Vec<i64>,
    phases: Vec<i64>,
) -> i64 {

    let (a_in, a_out, mut amp_a) = IntcodeMachine::new(program.to_owned());
    let (b_in, b_out, mut amp_b) = IntcodeMachine::new(program.to_owned());
    let (c_in, c_out, mut amp_c) = IntcodeMachine::new(program.to_owned());
    let (d_in, d_out, mut amp_d) = IntcodeMachine::new(program.to_owned());
    let (e_in, e_out, mut amp_e) = IntcodeMachine::new(program.to_owned());

    a_in.send(phases[0]).unwrap();
    b_in.send(phases[1]).unwrap();
    c_in.send(phases[2]).unwrap();
    d_in.send(phases[3]).unwrap();
    e_in.send(phases[4]).unwrap();

    a_in.send(0).unwrap();

    thread::spawn(move || {
        amp_a.run_program();
    });
    thread::spawn(move || {
        for i in a_out {
            b_in.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        amp_b.run_program();
    });
    thread::spawn(move || {
        for i in b_out {
            c_in.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        amp_c.run_program();
    });
    thread::spawn(move || {
        for i in c_out {
            d_in.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        amp_d.run_program();
    });
    thread::spawn(move || {
        for i in d_out {
            e_in.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        amp_e.run_program();
    });
    let final_out = thread::spawn(move || {
        let mut result: i64 = -1;
        for i in e_out {
            match a_in.send(i) {
                Ok(_) => {
                    continue;
                },
                Err(_) => {
                    result = i;
                    break;
                },
            }
        }
        result
    });

    final_out.join().unwrap()
}

fn heaps(n: usize, a: &mut Vec<i64>) -> Vec<Vec<i64>> {
    let mut ret: Vec<Vec<i64>> = Vec::new();
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

pub fn seven_a<I>(buf: I) -> i64
where
    I: BufRead,
{
    let p = intcode::read_program(buf);
    let mut phases = vec![0,1,2,3,4];
    let mut o = 0;
    let mut max = o;
    for v in heaps(5, &mut phases).iter() {
        o = amp_chain_feedback(p.to_owned(), v.to_vec());
        if o > max {
            max = o;
        }
    }
    max
}

pub fn seven_b<I>(buf: I) -> i64
where
    I: BufRead,
{
    let p = intcode::read_program(buf);
    let mut phases = vec![5,6,7,8,9];
    let mut o = 0;
    let mut max = o;
    for v in heaps(5, &mut phases).iter() {
        o = amp_chain_feedback(p.to_owned(), v.to_vec());
        if o > max {
            max = o;
        }
    }
    max
}


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

    #[test]
    fn test_seven_b() {
        let program = b"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(seven_b(&program[..]), 139629729);

        let program = b"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(seven_b(&program[..]), 18216);
    }
}
