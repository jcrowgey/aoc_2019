pub struct IntcodeMachine {
    ip: usize,
    program: Vec<usize>,
}

impl IntcodeMachine {
    pub fn new(p: Vec<usize>) -> IntcodeMachine {
        IntcodeMachine {
            ip: 0,
            program: p,
        }
    }
    pub fn run_program(&mut self) -> usize {
        self.ip = 0;
        loop {
            match self.program[self.ip] {
                1 => self.add(),
                2 => self.multiply(),
                99 => break,
                _ => panic!("bad input"),
            }
        }
        self.program[0]
    }

    pub fn add(&mut self) {
        let s = self.deref(self.ip + 1) + self.deref(self.ip + 2);
        let res_addr = self.program[self.ip + 3];
        self.program[res_addr] = s;
        self.ip += 4;
    }

    pub fn multiply(&mut self) {
        let p = self.deref(self.ip + 1) * self.deref(self.ip + 2);
        let res_addr = self.program[self.ip + 3];
        self.program[res_addr] = p;
        self.ip += 4;
    }

    fn deref(&mut self, addr: usize) -> usize {
        self.program[self.program[addr]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_machine() {
        let test_program: Vec<usize> = [1,9,10,3,2,3,11,0,99,30,40,50].to_vec();

        let mut mach = IntcodeMachine {
            ip: 0,
            program: test_program,
        };
        assert_eq!(3500, mach.run_program());
    }
}
