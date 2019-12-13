use std::io::BufRead;
use std::collections::VecDeque;

pub fn read_program<I>(mut buf: I) -> Vec<i32>
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
    out_buf: VecDeque<i32>,
    in_buf: VecDeque<i32>,
    memory: Vec<i32>,
}

impl IntcodeMachine {
    pub fn new(program: Vec<i32>) -> IntcodeMachine {
        IntcodeMachine {
            ip: 0,
            in_buf: VecDeque::new(),
            out_buf: VecDeque::new(),
            memory: program,
        }
    }

    pub fn input(&mut self, v: i32) {
        self.in_buf.push_front(v);
    }

    pub fn output(&mut self) -> Option<i32> {
        self.out_buf.pop_back()
    }

    pub fn run_program(&mut self) -> i32 {
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
                99 => break,
                _ => panic!("bad input"),
            }
        }
        self.memory[0] // exit code
    }

    fn parse_instruction(&mut self) -> (Vec<bool>, u8) {
        let mut mode = Vec::<bool>::new();
        let mut tmp = self.memory[self.ip];
        let opcode = (tmp % 100) as u8;
        tmp = tmp / 100;

        while tmp > 0 {
            mode.push((tmp % 10) > 0);
            tmp = tmp / 10;
        }
        while mode.len() < 3 {
            mode.push(false);
        }

        mode.reverse();
        (mode, opcode)
    }

    // Instruction implementations:
    fn add(&mut self, mode: Vec<bool>) {
        let params = self.eval_params(mode, 2);
        let sum = params[0] + params[1];
        let res_addr = self.addr(self.memory[self.ip + 3]);
        self.memory[res_addr] = sum;
        self.ip += 4;
    }

    fn mul(&mut self, mode: Vec<bool>) {
        let params = self.eval_params(mode, 2);
        let prod = params[0] * params[1];
        let res_addr = self.addr(self.memory[self.ip + 3]);
        self.memory[res_addr] = prod;
        self.ip += 4;
    }

    fn inp(&mut self, _mode: Vec<bool>) {
        let inp = self.read_input();
        let res_addr = self.addr(self.memory[self.ip + 1]);
        self.memory[res_addr] = inp;
        self.ip += 2;
    }

    fn out(&mut self, mode: Vec<bool>) {
        let out = self.eval_params(mode, 1);
        self.write_output(out[0]);
        self.ip += 2;
    }


    fn jit(&mut self, mut mode: Vec<bool>) {
        unimplemented!();
    }

    fn jif(&mut self, mut mode: Vec<bool>) {
        unimplemented!();
    }

    fn lt(&mut self, mut mode: Vec<bool>) {
        unimplemented!();
    }

    fn eq(&mut self, mut mode: Vec<bool>) {
        unimplemented!();
    }

    // Internal helpers 
    fn deref(&mut self, addr: usize) -> i32 {
        self.memory[self.memory[addr] as usize]
    }

    fn addr(&mut self, value: i32) -> usize {
        value as usize
    }

    fn read_input(&mut self) -> i32 {
        self.in_buf.pop_back().expect("i should block instead of panic")
    }

    fn write_output(&mut self, out: i32) {
        self.out_buf.push_front(out);
    }

    // returns a Vec of n values
    fn eval_params(&mut self, mut mode: Vec<bool>, n: usize) -> VecDeque<i32> {
        let mut ret = VecDeque::new();
        let mut i = 0;
        while n > i {
            let mut p: i32 = self.memory[self.ip + i + 1];
            let m: bool = mode.pop().expect("mode too short");
            if !m {
                p = self.deref(self.ip + i + 1);
            }
            i += 1;
            ret.push_front(p);
        }
        ret
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_machine_add_mul() {
        let test_program: Vec<i32> = [1,9,10,3,2,3,11,0,99,30,40,50].to_vec();

        let mut mach = IntcodeMachine::new(test_program);
        assert_eq!(3500, mach.run_program());
    }

    #[test]
    fn test_parse_instruction() {
        let mut mach = IntcodeMachine {
            ip: 0,
            in_buf: VecDeque::new(),
            out_buf: VecDeque::new(),
            memory: [1002].to_vec(),
        };
        assert_eq!(mach.parse_instruction(), ([false, true, false].to_vec(), 2));
    }
}
