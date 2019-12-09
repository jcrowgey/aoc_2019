pub struct IntcodeMachine {
    ip: usize,
    memory: Vec<i32>,
}

impl IntcodeMachine {
    pub fn new(program: Vec<i32>) -> IntcodeMachine {
        IntcodeMachine {
            ip: 0,
            memory: program,
        }
    }

    pub fn run_program(&mut self) -> i32 {
        self.ip = 0;
        loop {
            match self.memory[self.ip] {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(),
                4 => self.output(),
                99 => break,
                _ => panic!("bad input"),
            }
        }
        self.memory[0]
    }

    fn add(&mut self) {
        let s = self.deref(self.ip + 1) + self.deref(self.ip + 2);
        let res_addr = self.addr(self.memory[self.ip + 3]);
        self.memory[res_addr] = s;
        self.ip += 4;
    }

    fn multiply(&mut self) {
        let p = self.deref(self.ip + 1) * self.deref(self.ip + 2);
        let res_addr = self.addr(self.memory[self.ip + 3]);
        self.memory[res_addr] = p;
        self.ip += 4;
    }

    fn input(&mut self) {
        let inp = self.read_input();
        let res_addr = self.addr(self.memory[self.ip + 1]);
        self.memory[res_addr] = inp;
        self.ip += 2;
    }

    fn output(&mut self) {
        let out_addr = self.addr(self.memory[self.ip + 1]);
        let out = self.deref(out_addr);
        self.print_output(out);
        self.ip += 2;
    }

    fn deref(&mut self, addr: usize) -> i32 {
        self.memory[self.memory[addr] as usize]
    }

    fn addr(&mut self, value: i32) -> usize {
        value as usize
    }

    fn read_input(&mut self) -> i32 {
        unimplemented!();
    }

    fn print_output(&mut self, out: i32) {
        println!("{}", out);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_machine_add_mul() {
        let test_program: Vec<i32> = [1,9,10,3,2,3,11,0,99,30,40,50].to_vec();

        let mut mach = IntcodeMachine {
            ip: 0,
            memory: test_program,
        };
        assert_eq!(3500, mach.run_program());
    }
}
