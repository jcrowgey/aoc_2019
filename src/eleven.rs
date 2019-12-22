use std::io::BufRead;
use std::collections::HashMap;
use std::thread;
use crate::intcode;
use crate::point::Point;
use crate::intcode::IntcodeMachine;


fn run_robot(program: Vec<i64>, start_color: i64) -> HashMap<Point,i64> {
    let (m_in, m_out, mut mach) = IntcodeMachine::new(program);
    thread::spawn(move || {
        mach.run_program();
    });

    let mut location = Point{x: 0, y: 0};
    let mut panel = HashMap::new();
    panel.insert(location.clone(), 0);
    let directions: [fn(&Point) -> Point; 4] = [
        Point::up,
        Point::right,
        Point::down,
        Point::left,
    ];

    m_in.send(start_color as i64).unwrap();

    // initial direction is up, at 0
    let mut direction = 0;

    for o in &m_out {
        // paint
        panel.insert(location.clone(), o);

        // read move instruction
        match m_out.recv() {
            Ok(0) => {
                direction = (direction - 1) % 4;
            },
            Ok(1) => {
                direction = (direction + 1) % 4;
            },
            Ok(_) => panic!("gargage from machine"),
            Err(_) => panic!("can't read turn from robot"),
        }

        // move
        location = directions[direction](&location);

        // record panel color
        if !panel.contains_key(&location) {
            panel.insert(location.clone(), 0);
        }

        // send panel color
        m_in.send(*panel.get(&location).unwrap()).unwrap();

    }
    panel

}

fn draw_panel(panel: HashMap<Point, i64>) {
    // get the bounding box
    let mut y_min = i32::max_value();
    let mut y_max = i32::min_value();
    let mut x_min = i32::max_value();
    let mut x_max = i32::min_value();
    for point in panel.keys() {
        if point.y < y_min {
            y_min = point.y;
        }
        if point.y > y_max {
            y_max = point.y;
        }
        if point.x < x_min {
            x_min = point.x;
        }
        if point.x > x_max {
            x_max = point.x;
        }
    }

    for y in (y_min..y_max + 1).rev() {
        for x in x_min..x_max + 1 {
            let p = Point{x: x, y: y};
            if panel.contains_key(&p) {
                match panel.get(&p) {
                    Some(0) => print!(" "),
                    Some(1) => print!("▒"),
                    Some(_) => panic!("not a color"),
                    None => panic!("hashmap fail"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn eleven_a<I>(buf: I) -> usize
where
    I: BufRead,
{

    let p = intcode::read_program(buf);
    let panel = run_robot(p, 0);
    panel.len()
}


pub fn eleven_b<I>(buf: I) -> String
where
    I: BufRead,
{

    let p = intcode::read_program(buf);
    let panel = run_robot(p, 1);

    // draw panel
    draw_panel(panel);
    "[see above]".to_string()
}

