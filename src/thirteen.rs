use std::thread;
use std::io::{self, BufRead};
use std::sync::mpsc::{channel,Receiver,RecvError};
use crate::intcode;
use crate::intcode::IntcodeMachine;
use std::fs::{self};
use std::io::prelude::*;
use std::time;

use std::os::unix::io::AsRawFd;

use termios::*;
extern crate libc;

const SPRITES: [&str; 5] = [" ", "░", "▓", "━", "●",];
struct Screen {
    score: i32,
    x_buf: i32,
    y_buf: i32,
    tty: i32,
    orig_termios: Termios,
}

impl Screen {
    pub fn new(tty: i32) -> Screen {
        let termios = Termios::from_fd(tty).unwrap();
        Screen {
            score: 0,
            x_buf: 1,
            y_buf: 1,
            tty: tty,
            orig_termios: termios,
        }
    }

    pub fn cursor_to(&mut self, x: i32, y: i32) {
        print!("\x1b[{};{}H", y, x);
    }

    pub fn init(&mut self) {
        let mut termios = self.orig_termios.clone();
        cfmakeraw(&mut termios);
        tcsetattr(self.tty, TCSANOW, &termios)
            .expect("raw mode failed");
        print!("\x1b[?25l");  // hide cursor
        io::stdout().flush().unwrap();
    }

    pub fn done(&mut self) {
        self.cursor_to(11, 10);
        print!(" G A M E  O V E R ");
        self.cursor_to(0, 23);
        tcsetattr(self.tty, TCSADRAIN, &self.orig_termios)
            .expect("failed to restore terminal sanity; sorry!");
        print!("\x1b[?25h");  // restore cursor
        println!();
        io::stdout().flush().unwrap();
    }

    pub fn clear(&mut self) {
        print!("\x1b[2J");
        io::stdout().flush().unwrap();
    }

    pub fn draw(&mut self, xyt: [i32; 3]) {
        if xyt[0] == -1 {
            self.score = xyt[2];
            self.cursor_to(self.x_buf, 22 + self.y_buf);
            print!("Score: {:?}     ", self.score);  // spaces to clear the previous print on a loss
            return;
        }

        self.cursor_to(self.x_buf + xyt[0], self.y_buf + xyt[1]);
        print!("{}", SPRITES[xyt[2] as usize]);
        io::stdout().flush().unwrap();
    }
}


fn read_xyt(m_out: &Receiver<i64>) -> Result<[i32; 3],RecvError> {
    let x = m_out.recv()?;
    let y = m_out.recv()?;
    let tile_type = m_out.recv()?;
    Ok([x as i32, y as i32, tile_type as i32])
}


pub fn thirteen_a<I>(buf: I) -> usize
where
    I: BufRead,
{

    let program = intcode::read_program(buf);
    let (_, m_out, mut mach) = IntcodeMachine::new(program);
    thread::spawn(move || {
        mach.run_program();
    });

    let mut count = 0;
    loop {
        match read_xyt(&m_out) {
            Ok(xyt) => {
                if xyt[2] == 2 {
                    count += 1;
                }
            },
            Err(_) => {
                break;
            },
        }
    }
    count
}

pub fn thirteen_b<I>(buf: I) -> usize
where
    I: BufRead,
{

    let mut program = intcode::read_program(buf);
    program[0] = 2;  // free play!
    let (m_in, m_out, mut mach) = IntcodeMachine::new(program);
    thread::spawn(move || {
        mach.run_program();
    });
    let (x_sender, x_receiver) = channel();
    let game = thread::spawn(move || {
        let mut score = 0;
        loop {
            match read_xyt(&m_out) {
                Ok(xyt) => {
                    if xyt[0] == -1  { // score update
                        score = xyt[2];
                        continue;
                    }
                    if xyt[2] == 4 { // location ball update
                        x_sender.send(xyt[0]).unwrap();
                    }

                },
                Err(_) => {
                    break;
                },
            }
        }
        score
    });

    let mut paddle_x = 19;  // initial x position for paddle
    m_in.send(0).expect("failed to send initial joystic"); // joystick in neutral
    // ball falls 17, 18, 19, onto the paddle initial position
    // drain the first one from the channel in correspondence with the initial neutral
    x_receiver.recv().unwrap();  // pops 17
    loop {
        match x_receiver.recv() {
            Ok(ball_x) => {
                let mut joystick = 0;
                if ball_x < paddle_x {
                    joystick = -1;
                    paddle_x -= 1;
                } else if ball_x > paddle_x {
                    joystick = 1;
                    paddle_x += 1;
                }
                match m_in.send(joystick) {
                    Err(_) => { break; },
                    _ => { },
                }
            },
            _ => { break; },
        }
    }

    game.join().unwrap() as usize
}

pub fn play_interactive<I>(buf: I)
where
    I: BufRead,
{
    let mut program = intcode::read_program(buf);
    program[0] = 2;  // free play!
    let (m_in, m_out, mut mach) = IntcodeMachine::new(program);
    let tty = fs::OpenOptions::new().read(true)
                                    .write(true)
                                    .open("/dev/tty")
                                    .expect("failed to open /dev/tty");

    thread::spawn(move || mach.run_program());

    let mut screen = Screen::new(tty.as_raw_fd());
    thread::spawn(move || {
        screen.init();
        screen.clear();
        loop {
            match read_xyt(&m_out) {
                Ok(xyt) => {
                    screen.draw(xyt);
                },
                Err(_) => {
                    break;
                },
            }
        }
        screen.done();
        screen.score
    });

    m_in.send(0).expect("failed to send initial joystic"); // joystick in neutral
    let (tty_tx, tty_rx) = channel();
    loop {
        // len 8 to attempt to flush additional keystrokes
        let mut read_buf = [0u8;8];

        let tty_tx_local = tty_tx.clone();
        let mut tty_local = tty.try_clone().unwrap();
        thread::spawn(move || {
            tty_local.read(&mut read_buf).unwrap();
            &tty_tx_local.send(read_buf).unwrap();
        });

        // To give the game a constant feel, we try to read on a consistent schedule
        thread::sleep(time::Duration::from_millis(250));
        let mut joystick = 0;
        match tty_rx.try_recv() {
            Ok(input) => {
                if input[0] == 'j' as u8 {
                    joystick = -1;
                } else if input[0] == 'k' as u8 {
                    joystick = 1;
                }
            },
            Err(_) => {},
        }

        match m_in.send(joystick) {
            Err(_) => { break; },
            _ => {},
        };
    }
}
