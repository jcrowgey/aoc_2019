use std::io::BufRead;
use std::collections::{HashSet,HashMap};
use crate::point::Point;


fn map_wire(wire: String) -> HashSet<Point> {
    let mut cur = Point{
        x: 0,
        y: 0,
    };

    let mut points = HashSet::new();
    for vector in wire.trim().split(",").into_iter() {
        let (direction, magnitude) = vector.split_at(1);
        let mut magnitude = magnitude.parse::<i32>().unwrap();
        let f: fn(&Point) -> Point;
        match direction {
            "U" => f = Point::up,
            "D" => f = Point::down,
            "L" => f = Point::left,
            "R" => f = Point::right,
            _ => panic!("bad input: {:?}", direction),
        }
        while magnitude > 0 {
            cur = f(&cur);
            points.insert(cur.to_owned());
            magnitude -= 1;
        }
    }
    points
}

fn map_wire_dist(wire: String) -> HashMap<Point, i32> {
    let mut cur = Point{
        x: 0,
        y: 0,
    };

    let mut points = HashMap::new();
    let mut i: i32 = 0;
    for vector in wire.trim().split(",").into_iter() {
        let (direction, magnitude) = vector.split_at(1);
        let mut magnitude = magnitude.parse::<i32>().unwrap();
        let f: fn(&Point) -> Point;
        match direction {
            "U" => f = Point::up,
            "D" => f = Point::down,
            "L" => f = Point::left,
            "R" => f = Point::right,
            _ => panic!("bad input: {:?}", direction),
        }
        while magnitude > 0 {
            i += 1;
            cur = f(&cur);
            if None == points.get(&cur) {
                points.insert(cur.to_owned(), i);
            }
            magnitude -= 1;
        }
    }
    points
}

pub fn three_a<I>(mut buf: I) -> i32
where
    I: BufRead,
{
    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    let w1_points = map_wire(line);

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    let w2_points = map_wire(line);

    let mut inter: Vec<_> = w1_points.intersection(&w2_points).collect();

    inter.sort();
    inter[0].dist()
}

pub fn three_b<I>(mut buf: I) -> i32
where
    I: BufRead,
{
    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    let mut w1_points = map_wire_dist(line);

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    let w2_points = map_wire_dist(line);

    w1_points.retain(|k, _| w2_points.get(&k) != None);
    let mut best  = i32::max_value();
    for (point, steps) in w1_points.iter() {
        let total = steps + w2_points.get(&point).unwrap();
        if total < best {
            best = total;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    static CASE0: &[u8; 23] = b"R8,U5,L5,D3\nU7,R6,D4,L4";
    static CASE1: &[u8; 66] = b"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    static CASE2: &[u8; 80] = b"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_three_a() {
        assert_eq!(three_a(&CASE0[..]), 6);
        assert_eq!(three_a(&CASE1[..]), 159);
        assert_eq!(three_a(&CASE2[..]), 135);
    }

    #[test]
    fn test_three_b() {
        assert_eq!(three_b(&CASE0[..]), 30);
        assert_eq!(three_b(&CASE1[..]), 610);
        assert_eq!(three_b(&CASE2[..]), 410);
    }
}
