use std::io::BufRead;
use std::collections::{HashSet, HashMap};
use crate::point::Point;

fn gcd(x: i32, y: i32) -> i32 {
    if x == y {
        x
    } else {
        let min = if x < y {x} else {y};
        gcd(min, (x - y).abs())
    }
}

#[derive(Clone,Hash,Debug)]
struct Rational {
    num: i32,
    den: i32,
}

impl Rational {
    fn reduce(&mut self) -> Result<(), i32> {
        if self.num == 0 && self.den == 0 {
            return Err(-1)
        }

        if self.num == 0 {
            self.den /= self.den.abs();  // preserve sign
            return Ok(())
        }

        if self.den == 0 {
            self.num /= self.num.abs();  // preserve sign
            return Ok(())
        }

        if self.num == 1 || self.den == 1 {
            return Ok(())
        }

        // store signs then restore them
        let num_neg = self.num < 0;
        let den_neg = self.den < 0;

        self.num = self.num.abs();
        self.den = self.den.abs();

        let gcd = gcd(self.num, self.den);

        if num_neg {
            self.num = self.num * -1;
        }
        if den_neg {
            self.den = self.den * -1;
        }

        self.num = self.num / gcd;
        self.den = self.den / gcd;
        Ok(())
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.den == other.den
    }
}

fn ob(p: &Point, xdim: usize, ydim: usize) -> bool {
    p.x > (xdim as i32) || p.y > (ydim as i32) || p.x < 0 || p.y < 0
}

// Three points on a line, caller guarantees collinearity
fn is_between(center: &Point, this: &Point, that: &Point) -> bool {
    if (center.x - this.x).abs() < (that.x - this.x).abs() {
        return true;
    } else if (center.y - this.y).abs() < (that.y - this.y).abs() {
        return true;
    }
    return false;
}

fn can_see(p: &Point, q: &Point, locations: &HashSet<Point>, xdim: usize, ydim: usize) -> bool {
    // check_point is from the standpoint of p
    // so that slope is just the reduction of q
    let mut slope = Rational{
        num: q.x - p.x,
        den: q.y - p.y,
    };

    slope.reduce().unwrap();

    // now, while our coords are within the graph, keep moving outward
    let mut check_point = Point{ x: slope.num, y: slope.den};
    let mut i = 1;
    loop {
        // but use the real location for the checks
        let real_point = Point{x: check_point.x + p.x, y: check_point.y + p.y};

        if ob(&real_point, xdim, ydim) {
            return false
        }

        if locations.contains(&real_point) {
            if check_point == *q {
                return true
            }

            if is_between(&real_point, &p, &q) {
                return false
            } else {
                return true
            }
        }

        i += 1;
        check_point = Point{ x: slope.num * i, y: slope.den * i };
    }
}

pub fn ten_a<I>(buf: I) -> usize
where
    I: BufRead,
{
    let mut locations = HashSet::new();
    let mut xdim = 0;
    let mut ydim = 0;

    for (y, line) in buf.lines().enumerate() {
        ydim += 1;
        let line = line.unwrap();
        for (x, ch) in line.chars().enumerate() {
            if y == 0 {
                xdim += 1;
            }
            if ch == '#' {
                locations.insert(Point{x: x as i32, y: y as i32});
            }
        }
    }

    let mut relations = HashMap::<(&Point,&Point),bool>::new();
    let mut viz_counts = HashMap::<&Point,usize>::new();
    for p in locations.iter() {
        for q in locations.iter() {
            if *p == *q {
                continue;
            }
            if relations.contains_key(&(q, p)) {
                continue;
            }
            let v = can_see(p, q, &locations, xdim, ydim);
            relations.insert((p, q), v);
            relations.insert((q, p), v);
            if v {
                if let Some(pcount) = viz_counts.get_mut(&p) {
                    *pcount += 1;
                } else {
                    viz_counts.insert(p, 1);
                }
                if let Some(qcount) = viz_counts.get_mut(&q) {
                    *qcount += 1;
                } else {
                    viz_counts.insert(q, 1);
                }
            }
        }
    }

    let mut count_vec: Vec<usize> = viz_counts.values().cloned().collect();
    count_vec.sort_unstable();
    count_vec[count_vec.len() - 1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ten_a() {
        let input = b".#.\n.#.\n.#.\n";
        assert_eq!(ten_a(&input[..]), 2);

        let input = b".#..#\n.....\n#####\n....#\n...##";
        assert_eq!(ten_a(&input[..]), 8);

        let input = b".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(ten_a(&input[..]), 210);
    }

    #[test]
    fn test_reduce_rational() {
        let mut rat = Rational{num: 2, den: 4};
        rat.reduce().expect("failed to reduce rational");
        assert_eq!(rat.num, 1);
        assert_eq!(rat.den, 2);

        rat = Rational{num: 2, den: 0};
        rat.reduce().expect("failed to reduce rational");
        assert_eq!(rat.num, 1);
        assert_eq!(rat.den, 0);

        rat = Rational{num: -2, den: 0};
        rat.reduce().expect("failed to reduce rational");
        assert_eq!(rat.num, -1);
        assert_eq!(rat.den, 0);

        rat = Rational{num: -1, den: 5};
        rat.reduce().expect("failed to reduce rational");
        assert_eq!(rat.num, -1);
        assert_eq!(rat.den, 5);
    }

    #[test]
    fn test_is_between() {
        let a = Point{x:0,y:0};
        let b = Point{x:1,y:0};
        let c = Point{x:2,y:0};
        let res = is_between(&a, &b, &c);
        assert!(!res);
        let res = is_between(&c, &a, &b);
        assert!(!res);
        let res = is_between(&b, &a, &c);
        assert!(res);

        let a = Point{x:0,y:1};
        let b = Point{x:0,y:2};
        let c = Point{x:0,y:3};
        let res = is_between(&a, &b, &c);
        assert!(!res);
        let res = is_between(&c, &a, &b);
        assert!(!res);
        let res = is_between(&b, &a, &c);
        assert!(res);

        let a = Point{x:0,y:0};
        let b = Point{x:1,y:1};
        let c = Point{x:2,y:2};
        let res = is_between(&a, &b, &c);
        assert!(!res);
        let res = is_between(&c, &a, &b);
        assert!(!res);
        let res = is_between(&b, &a, &c);
        assert!(res);
    }
}
