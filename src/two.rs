use std::io::BufRead;

struct IntcodeMachine {
    ip: usize,
    program: Vec<usize>,
}

impl IntcodeMachine {
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

    let p = read_program(buf);
    let mut mach = IntcodeMachine {
        ip: 0,
        program: p,
    };

    mach.program[1] = 12;
    mach.program[2] = 2;

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
    loop {
        if noun == 100 {
            break;
        }
        let mut verb: usize = 0;
        'inner: loop {
            if verb == 100 {
                break 'inner;
            }
            let mut mach = IntcodeMachine {
                ip: 0,
                program: p.to_owned(),
            };
            mach.program[1] = noun;
            mach.program[2] = verb;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incode_machine() {
        let test_program: Vec<usize> = [1,9,10,3,2,3,11,0,99,30,40,50].to_vec();

        let mut mach = IntcodeMachine {
            ip: 0,
            program: test_program,
        };
        assert_eq!(3500, mach.run_program());
    }
}
