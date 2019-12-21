use std::io::BufRead;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering::Less;
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

impl Eq for Rational {}


fn next_point_on_line(slope: &Rational, base: &Point, locations: &HashSet<Point>,) -> Option<Point>
{
    let mut i = 1;
    loop {
        let relative_point = Point{
            x: slope.num * i,
            y: slope.den * i,
        };
        let real_point = Point{
            x: relative_point.x + base.x,
            y: (-1*relative_point.y) + base.y,
        };
        if locations.contains(&real_point) {
            return Some(real_point);
        }
        i += 1;
    }

}

fn can_see(p: &Point, q: &Point, locations: &HashSet<Point>) -> bool {
    let mut slope = Rational{
        num: q.x - p.x,
        den: -1 * (q.y - p.y),
    };

    slope.reduce().unwrap();

    match next_point_on_line(&slope, &p, &locations) {
        Some(next_point) => {
            if next_point == *q {
                return true;
            }
            return false;
        },
        _ => return false,
    }
}

fn get_viz_counts(locations: &HashSet<Point>) -> HashMap::<&Point, usize>
{
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
            let v = can_see(p, q, &locations);
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
    viz_counts
}

fn read_input<I>(buf: I) -> HashSet<Point>
where
    I: BufRead,
{
    let mut locations = HashSet::new();

    for (y, line) in buf.lines().enumerate() {
        let line = line.unwrap();
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                locations.insert(Point{x: x as i32, y: y as i32});
            }
        }
    }
    locations
}

fn get_slopes(base: &Point, locations: &HashSet<Point>) -> Vec<Rational>
{

    let mut quadrants: Vec<HashSet<Rational>> = Vec::new();
    for _ in 0..4 {
        quadrants.push(HashSet::<Rational>::new());
    }

    for loc in locations.iter() {
        let mut rel = Rational{num: loc.x-base.x, den: -1 * (loc.y - base.y)};
        if rel.num ==  0 && rel.den == 0 {
            continue;
        }
        rel.reduce().unwrap();

        if rel.num >= 0 && rel.den > 0 {
            quadrants[0].insert(rel);
        } else if rel.num >= 0 {
            quadrants[1].insert(rel);
        } else if rel.den <= 0 {
            quadrants[2].insert(rel);
        } else {
            quadrants[3].insert(rel);
        }
    }

    // these sorted slopes are funny biz
    let slope_sort = |a: &Rational, b: &Rational| {
        (a.num as f64/a.den as f64)
            .partial_cmp(&(b.num as f64/b.den as f64))
            .unwrap_or(Less)
    };
    let mut sorted_slopes: Vec<Rational> = Vec::new();

    for q in quadrants {
        let mut sorted_q: Vec<_> = q.iter().cloned().collect();
        sorted_q.sort_by(slope_sort);
        sorted_slopes.append(&mut sorted_q);
    }

    sorted_slopes
}

fn get_destroy_order(
    mut locations: HashSet<Point>,
    base_location: &Point,
    stop: usize,
) -> Vec<Point> {

    let mut res = Vec::new();
    let slopes = get_slopes(base_location, &locations);
    let mut i = 0;
    while res.len() < stop {
        let slope_idx = i % slopes.len();
        let laser_slope: &Rational = &slopes[slope_idx];
        if let Some(p) = next_point_on_line(&laser_slope, &base_location, &locations) {
            locations.remove(&p);
            res.push(p);
        }

        i += 1;
    }
    res
}

pub fn ten_a<I>(buf: I) -> usize
where
    I: BufRead,
{

    let locations = read_input(buf);
    let viz_counts = get_viz_counts(&locations);
    let mut count_vec: Vec<usize> = viz_counts.values().cloned().collect();
    count_vec.sort_unstable();
    count_vec[count_vec.len() - 1]
}

pub fn ten_b<I>(buf: I) -> usize
where
    I: BufRead,
{
    let locations = read_input(buf);
    let viz_counts = get_viz_counts(&locations);
    let mut by_count: Vec<_> = viz_counts.iter().collect();
    by_count.sort_by(|a, b| a.1.cmp(b.1).reverse());
    let base_location = by_count[0].0.clone();
    drop(viz_counts);

    let destroy_order = get_destroy_order(locations.to_owned(), &base_location, 200);
    destroy_order[199].x as usize * 100 + (destroy_order[199].y) as usize
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
    fn test_ten_b() {
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
        assert_eq!(ten_b(&input[..]), 802);
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
}
