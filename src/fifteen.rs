use std::fmt;
use std::string::String;
use std::io::BufRead;
use std::collections::HashMap;
use std::thread;
use crate::intcode::{self, IntcodeMachine};
use crate::point::Point;

#[derive(Debug)]
struct BoundingBox {
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
}

impl BoundingBox {
    fn expand(&mut self, p: &Point) {
        if p.x > self.max_x {
            self.max_x = p.x;
        }
        if p.x < self.min_x {
            self.min_x = p.x;
        }
        if p.y > self.max_y {
            self.max_y = p.y;
        }
        if p.y < self.min_y {
            self.min_y = p.y;
        }
    }

    fn print_hborder(&self) {
        print!("+");
        for _ in self.min_x .. self.max_x + 1 {
            print!("-");
        }
        print!("+\n");
    }
}


#[derive(Debug)]
struct Space {
    bbox: BoundingBox,
    points: HashMap<Point,String>,
    target: Option<Point>,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bbox.print_hborder();
        for y in (self.bbox.min_y..self.bbox.max_y + 1).rev() {
            write!(f, "|").unwrap();
            for x in self.bbox.min_x .. self.bbox.max_x + 1 {
                if x == 0 && y == 0 {
                    write!(f, "O").unwrap();
                } else if self.target != None
                          && x == self.target.as_ref().unwrap().x
                          && y == self.target.as_ref().unwrap().y {
                    write!(f, "X").unwrap();
                } else {
                    match self.points.get(&Point{x,y}) {
                        Some(t) => {
                            write!(f, "{:}", t).unwrap();
                        },
                        None => {
                            write!(f, " ").unwrap();
                        }
                    }
                }
            }
            write!(f, "|\n").unwrap();
        }
        self.bbox.print_hborder();
        Ok(())
    }

}

fn explore_space<I>(buf: I) -> Space
where
    I: BufRead
{
    let program = intcode::read_program(buf);
    let (m_in, m_out, mut mach) = IntcodeMachine::new(program);
    let bbox = BoundingBox{
        max_x: 0,
        min_x: 0,
        max_y: 0,
        min_y: 0,
    };
    let directions: [(fn(&Point) -> Point, i64); 4] = [
        (Point::up, 1),
        (Point::left, 3),
        (Point::down, 2),
        (Point::right, 4),
    ];
    let mut dir: usize = 0;
    let mut loc = Point{x: 0, y: 0};
    let mut space = Space {
        bbox: bbox,
        points: HashMap::new(),
        target: None,
    };
    space.points.insert(loc.to_owned(), ".".to_string());

    thread::spawn(move || {
        mach.run_program();
    });

    loop {
        let next_loc = directions[dir].0(&loc);
        if next_loc.x == 0 && next_loc.y == 0 {
            // The machine is blocking on an inp instruction
            // We send an invalid input command so that the program halts
            // rather than throwing a panic on recv err 
            m_in.send(-1).unwrap();
            break;
        }
        m_in.send(directions[dir].1).unwrap();
        space.bbox.expand(&next_loc);
        match m_out.recv().unwrap() {
            0 => { // wall
                // turn back to the right
                space.points.insert(next_loc.to_owned(), "#".to_string());
                dir = (dir + 3) % 4;
            },
            o @ 1 | o @ 2 => { // forward 
                // try to turn left
                if o == 2 {
                    space.target = Some(next_loc.to_owned());
                }
                space.points.insert(next_loc.to_owned(), ".".to_string());
                loc = next_loc;
                dir = (dir + 1) % 4;
            },
            _ => {
                panic!("unexpected output");
            }
        }
    }
    // print!("{}", space);
    space
}

fn visit(point: &Point, pdist:usize, distances: &mut HashMap<Point,usize>) {
    for dir in [Point::up, Point::left, Point::down, Point::right].iter() {
        let neighbor = dir(point);
        if let Some(n) = distances.get_mut(&neighbor) {
            if *n > pdist {
                *n = pdist + 1;
            }
        }
    }
}

fn next(distances: &HashMap<Point,usize>) -> Option<Point> {
    let mut by_dist: Vec<_> = distances.iter().collect();
    by_dist.sort_by(|a, b| b.1.cmp(a.1));
    if by_dist.len() > 0 {
        Some(by_dist[by_dist.len()-1].0.to_owned())
    } else {
        None
    }
}

fn dikstras(space: &Space, origin: Point, target: Option<Point>) -> HashMap<Point,usize> {
    let mut distances: HashMap<Point,usize> = HashMap::from(
        space.points.iter()
                    .filter(|(_, v)| *v != &"#")
                    .map(
                        |(k, _)| {
                            (k.to_owned(), usize::max_value())
                        }
                    ).collect()
    );
    let mut visited = HashMap::new();

    let mut cur = origin;
    *distances.get_mut(&cur).expect("0 0 isn't here!") = 0;

    loop {
        let pdist = distances.get(&cur).unwrap().to_owned();
        visit(&cur, pdist, &mut distances);
        visited.insert(cur.to_owned(), pdist);
        distances.remove(&cur);
        match next(&distances) {
            Some(n) => {
                if let Some(t) = target.as_ref() {
                    if n == *t {
                        let pdist = distances.get(t).unwrap().to_owned();
                        visited.insert(t.to_owned(), pdist);
                        break;
                    }
                }
                cur = n;
            },
            None => {
                break;
            }
        }
    }
    visited
}

pub fn fifteen_a<I>(buf: I) -> usize
where
    I: BufRead
{
    let space = explore_space(buf);
    let origin = Point{x:0,y:0};
    let distances = dikstras(&space, origin.clone(), space.target.clone());
    *distances.get(&space.target.unwrap()).unwrap()
}

pub fn fifteen_b<I>(buf: I) -> usize
where
    I: BufRead
{
    let space = explore_space(buf);
    let distances = dikstras(&space, space.target.clone().unwrap(), None);
    let mut by_dist: Vec<_> = distances.iter().collect();
    by_dist.sort_by(|a, b| b.1.cmp(a.1));
    *by_dist[0].1
}
