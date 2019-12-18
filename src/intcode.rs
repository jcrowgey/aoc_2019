use std::io::BufRead;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Sender, Receiver};

pub fn read_program<I>(mut buf: I) -> Vec<i64>
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


pub struct IntcodeMachine {
    ip: usize,
    off: i64,
    out_tx: Sender<i64>,
    in_rx: Receiver<i64>,
    memory: Vec<i64>,
}

#[derive(Debug)]
enum Mode {
    Pointer,
    Value,
    Relative,
}

impl IntcodeMachine {
    pub fn new(mut program: Vec<i64>) -> (Sender<i64>, Receiver<i64>, IntcodeMachine) {
        let (itx, irx) = channel::<i64>();
        let (otx, orx) = channel::<i64>();
        let addl_mem = vec![0i64; 0xffff - program.len()];
        program.extend(addl_mem);
        let mach = IntcodeMachine {
            ip: 0,
            off: 0,
            in_rx: irx,
            out_tx: otx,
            memory: program,
        };
        (itx, orx, mach)
    }

    pub fn run_program(&mut self) -> i64 {
        self.ip = 0;
        loop {
            let (mode, opcode) = self.parse_instruction();
            match opcode {
                1 => self.add(mode),
                2 => self.mul(mode),
                3 => self.inp(mode),
                4 => self.out(mode),
                5 => self.jit(mode),
                6 => self.jif(mode),
                7 => self.lt(mode),
                8 => self.eq(mode),
                9 => self.rbo(mode),
                99 => break,
                _ => panic!("bad input"),
            }
        }
        self.memory[0] // exit code
    }

    fn parse_instruction(&mut self) -> (Vec<Mode>, u8) {
        let mut mode = Vec::<Mode>::new();
        let mut tmp = self.memory[self.ip];
        let opcode = (tmp % 100) as u8;
        tmp = tmp / 100;

        while tmp > 0 {
            match tmp % 10 {
                0 => mode.push(Mode::Pointer),
                1 => mode.push(Mode::Value),
                2 => mode.push(Mode::Relative),
                _ => panic!("bad mode"),
            }
            tmp = tmp / 10;
        }
        while mode.len() < 3 {
            mode.push(Mode::Pointer);
        }

        mode.reverse();
        (mode, opcode)
    }

    // Instruction implementations:
    fn add(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 2);
        let sum = params[0] + params[1];
        let res_addr = self.eval_write_param(mode.pop().unwrap(), self.ip + 3);
        self.memory[res_addr] = sum;
        self.ip += 4;
    }

    fn mul(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 2);
        let prod = params[0] * params[1];
        let res_addr = self.eval_write_param(mode.pop().unwrap(), self.ip + 3);
        self.memory[res_addr] = prod;
        self.ip += 4;
    }

    fn inp(&mut self, mut mode: Vec<Mode>) {
        let res_addr = self.eval_write_param(mode.pop().unwrap(), self.ip + 1);
        let inp = self.read_input();
        self.memory[res_addr] = inp;
        self.ip += 2;
    }

    fn out(&mut self, mut mode: Vec<Mode>) {
        let out = self.eval_params(&mut mode, 1);
        self.write_output(out[0]);
        self.ip += 2;
    }


    fn jit(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 2);
        if params[0] != 0 {
            self.ip = self.addr(params[1]);
        } else {
            self.ip += 3;
        }

    }

    fn jif(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 2);
        if params[0] == 0 {
            self.ip = self.addr(params[1]);
        } else {
            self.ip += 3;
        }
    }

    fn lt(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 2);
        let res_addr = self.eval_write_param(mode.pop().unwrap(), self.ip + 3);
        if params[0] < params[1] {
            self.memory[res_addr] = 1;
        } else {
            self.memory[res_addr] = 0;
        }
        self.ip += 4;
    }

    fn eq(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 2);
        let res_addr = self.eval_write_param(mode.pop().unwrap(), self.ip + 3);
        if params[0] == params[1] {
            self.memory[res_addr] = 1;
        } else {
            self.memory[res_addr] = 0;
        }
        self.ip += 4;
    }

    fn rbo(&mut self, mut mode: Vec<Mode>) {
        let params = self.eval_params(&mut mode, 1);
        self.off += params[0];
        self.ip += 2;
    }

    // Internal helpers 
    fn deref(&mut self, addr: usize, off: i64) -> i64 {
        let ptr = self.memory[addr];
        self.memory[(ptr + off) as usize]
    }

    fn addr(&mut self, value: i64) -> usize {
        value as usize
    }

    fn read_input(&mut self) -> i64 {
        self.in_rx.recv().unwrap()
    }

    fn write_output(&mut self, out: i64) {
        self.out_tx.send(out).expect("unable to send output");
    }

    // returns a Vec of n values
    fn eval_params(&mut self, mode: &mut Vec<Mode>, n: usize) -> VecDeque<i64> {
        let mut ret = VecDeque::new();
        let mut i = 0;
        while n > i {
            let base = self.ip + i + 1;
            match mode.pop() {
                Some(Mode::Value) => {
                    ret.push_back(self.memory[base]);
                },
                Some(Mode::Pointer) => {
                    ret.push_back(self.deref(base, 0));
                },
                Some(Mode::Relative) => {
                    ret.push_back(self.deref(base, self.off));
                },
                None => panic!("mode too short"),
            }
            i += 1;
        }
        ret
    }

    fn eval_write_param(&mut self, mode: Mode, addr: usize) -> usize {
        match mode {
            Mode::Value => panic!("cannot calculate write param in value mode"),
            Mode::Pointer => self.addr(self.memory[addr]),
            Mode::Relative => {
                self.addr(self.memory[addr] + self.off)
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_intcode_machine_add_mul() {
        let test_program: Vec<i64> = [1,9,10,3,2,3,11,0,99,30,40,50].to_vec();

        let (_m_in, _m_out, mut mach) = IntcodeMachine::new(test_program);
        assert_eq!(3500, mach.run_program());
    }

    #[test]
    fn test_eq() {
        let eq_8_ptr: Vec<i64> = [3,9,8,9,10,9,4,9,99,-1,8].to_vec();
        let (m_in, m_out, mut mach) = IntcodeMachine::new(eq_8_ptr.to_owned());
        let computation = thread::spawn(move || {
            mach.run_program()
        });
        m_in.send(7).expect("failed to send");
        let _exit_code = computation.join().unwrap();
        assert_eq!(0, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(eq_8_ptr.to_owned());
        let computation = thread::spawn(move || {
            mach.run_program()
        });
        m_in.send(8).expect("failed to send");
        let _exit_code = computation.join().unwrap();
        assert_eq!(1, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(eq_8_ptr.to_owned());
        let computation = thread::spawn(move || {
            mach.run_program()
        });
        m_in.send(9).expect("failed to send");
        let _exit_code = computation.join().unwrap();
        assert_eq!(0, m_out.recv().unwrap());


        let eq_8_val: Vec<i64> = [3,3,1108,-1,8,3,4,3,99].to_vec();
        let (m_in, m_out, mut mach) = IntcodeMachine::new(eq_8_val.to_owned());
        let computation = thread::spawn(move || {
            mach.run_program()
        });
        m_in.send(7).expect("failed to send");
        let _exit_code = computation.join().unwrap();
        assert_eq!(0, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(eq_8_val.to_owned());
        let computation = thread::spawn(move || {
            mach.run_program()
        });
        m_in.send(8).expect("failed to send");
        let _exit_code = computation.join().unwrap();
        assert_eq!(1, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(eq_8_val);
        let computation = thread::spawn(move || {
            mach.run_program()
        });
        m_in.send(9).expect("failed to send");
        let _exit_code = computation.join().unwrap();
        assert_eq!(0, m_out.recv().unwrap());
    }


    #[test]
    fn test_lt_instruction() {
        let lt_8_ptr: Vec<i64> = [3,9,7,9,10,9,4,9,99,-1,8].to_vec();
        let (m_in, m_out, mut mach) = IntcodeMachine::new(lt_8_ptr.to_owned());
        m_in.send(7).expect("failed to send");
        mach.run_program();
        assert_eq!(1, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(lt_8_ptr.to_owned());
        m_in.send(8).expect("failed to send");
        mach.run_program();
        assert_eq!(0, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(lt_8_ptr.to_owned());
        m_in.send(9).expect("failed to send");
        mach.run_program();
        assert_eq!(0, m_out.recv().unwrap());

        let lt_8_val: Vec<i64> = [3,3,1107,-1,8,3,4,3,99].to_vec();
        let (m_in, m_out, mut mach) = IntcodeMachine::new(lt_8_val.to_owned());
        m_in.send(7).expect("failed to send");
        mach.run_program();
        assert_eq!(1, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(lt_8_val.to_owned());
        m_in.send(8).expect("failed to send");
        mach.run_program();
        assert_eq!(0, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(lt_8_val.to_owned());
        m_in.send(9).expect("failed to send");
        mach.run_program();
        assert_eq!(0, m_out.recv().unwrap());
    }


    #[test]
    fn test_jmp() {
        let not_zero_ptr: Vec<i64> = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9].to_vec();
        let not_zero_val: Vec<i64> = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1].to_vec();

        let (m_in, m_out, mut mach) = IntcodeMachine::new(not_zero_ptr.to_owned());
        m_in.send(0).expect("failed to send");
        mach.run_program();
        assert_eq!(0, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(not_zero_ptr.to_owned());
        m_in.send(-11).expect("failed to send");
        mach.run_program();
        assert_eq!(1, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(not_zero_val.to_owned());
        m_in.send(0).expect("failed to send");
        mach.run_program();
        assert_eq!(0, m_out.recv().unwrap());

        let (m_in, m_out, mut mach) = IntcodeMachine::new(not_zero_val.to_owned());
        m_in.send(-1).expect("failed to send");
        mach.run_program();
        assert_eq!(1, m_out.recv().unwrap());
    }

    #[test]
    fn test_quine() {
        let quine: Vec<i64> = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99].to_vec();
        let (m_in, m_out, mut mach) = IntcodeMachine::new(quine.to_owned());
        for &b in quine.iter() {
            m_in.send(b).unwrap();
        }
        thread::spawn(move || {
            mach.run_program();
        });
        let mut i = 0;
        for b in m_out {
            assert_eq!(quine[i], b);
            i += 1;
        }
    }

    #[test]
    fn test_big_output() {
        let big_o: Vec<i64> = [1102,34915192,34915192,7,4,7,99,0].to_vec();
        let (_m_in, m_out, mut mach) = IntcodeMachine::new(big_o.to_owned());
        mach.run_program();
        let o = m_out.recv().unwrap();
        assert!(o > 999_999_999_999_999);
    }

    #[test]
    fn test_big_io() {
        let big_io: Vec<i64> = [104,1125899906842624,99].to_vec();
        let (_m_in, m_out, mut mach) = IntcodeMachine::new(big_io.to_owned());
        mach.run_program();
        let o = m_out.recv().unwrap();
        assert!(o == big_io[1]);
    }
}
