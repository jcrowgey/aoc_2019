use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug,Hash,Clone,Copy,PartialEq,Eq)]
struct Body {
    pos: [i32; 3],
    vel: [i32; 3],
}

#[derive(Debug,Hash,Clone,Copy,PartialEq,Eq)]
struct Term {
    base: u32,
    exp: u32,
}


fn prime_fact(n: u32) -> Vec<Term> {
    let mut res = Vec::new();
    let mut tmp_n = n;
    for d in 2..n {
        let mut exp = 0;
        while tmp_n % d == 0 {
            exp += 1;
            tmp_n /= d;
            if tmp_n == 1 {
                res.push(Term{base: d, exp: exp});
                return res;
            }
        }
        if exp > 0 {
            res.push(Term{base: d, exp: exp });
        }

        if d * d > n {
            res.push(Term{base: tmp_n, exp: 1});
            return res;
        }
    }
    unreachable!("what happened?!");
}

fn gravity(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            for axis in 0..3 {
                if bodies[i].pos[axis] < bodies[j].pos[axis] {
                    bodies[i].vel[axis] += 1;
                    bodies[j].vel[axis] -= 1;
                } else if bodies[i].pos[axis] > bodies[j].pos[axis] {
                    bodies[i].vel[axis] -= 1;
                    bodies[j].vel[axis] += 1;
                }
            }
        }
    }
}

fn velocity(bodies: &mut Vec<Body>) {
    for b in bodies {
        for axis in 0..3 {
            b.pos[axis] += b.vel[axis];
        }
    }
}

fn energy(bodies: Vec<Body>) -> i32 {
    let mut e = 0;
    for b in bodies {
        let mut pot = 0;
        let mut kin = 0;
        for axis in 0..3 {
            pot += b.pos[axis].abs();
            kin += b.vel[axis].abs();
        }
        e += pot * kin;
    }
    e
}

fn read_locations<I>(buf: I) -> Vec<Body>
where
    I: BufRead
{
    let mut bodies = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        let line = line.trim_matches(
            |c| c == '<' || c == '>'
        );

        let axes: Vec<_> = line.split(",").collect();
        assert_eq!(axes.len(), 3);
        let mut b = Body{
            pos: [0,0,0],
            vel: [0,0,0],
        };

        for a in axes {
            let k_v: Vec<_>  = a.splitn(2, "=").collect();
            let axis = k_v[0].trim();
            let magnitude = k_v[1].trim();
            match axis {
                "x" => b.pos[0] = magnitude.parse().expect("failed parsing magnitude"),
                "y" => b.pos[1] = magnitude.parse().expect("failed parsing magnitude"),
                "z" => b.pos[2]= magnitude.parse().expect("failed parsing magnitude"),
                _ => panic!("unknown axis"),
            }
        }
        bodies.push(b);
    }
    bodies
}

fn sim(mut bodies: Vec<Body>, steps: u32) -> i32 {
    for _ in 0..steps {
        gravity(&mut bodies);
        velocity(&mut bodies);
    }
    energy(bodies)
}

fn to_axis(bodies: &mut Vec<Body>, axis: usize) -> Vec<i32> {
    let mut res = Vec::new();
    for b in bodies {
        res.push(b.pos[axis]);
        res.push(b.vel[axis]);
    }
    res
}

fn find_periods(mut bodies: Vec<Body>) -> [u32; 3] {
    let mut counter = 0;
    let mut init_axes: Vec<_> = Vec::new();
    for axis in 0..3 {
        init_axes.push(to_axis(&mut bodies, axis));
    }
    let mut periods = [0,0,0];
    loop {
        counter += 1;
        gravity(&mut bodies);
        velocity(&mut bodies);

        for axis in 0..3 {
            if periods[axis] == 0 {
                let cur_axis = to_axis(&mut bodies, axis);
                if cur_axis == init_axes[axis] {
                    periods[axis] = counter;
                }
            }
        }

        if periods[0] > 0 && periods[1] > 0 && periods[2] > 0 {
            break;
        }
    }
    periods
}

fn sim_cycle(bodies: Vec<Body>) -> u64 {
    let periods = find_periods(bodies);
    let mut harmony = HashMap::new();
    for p in periods.iter() {
        let pf = prime_fact(*p);
        for term in pf {
            if let Some(v) = harmony.get_mut(&term.base) {
                if *v < term.exp {
                    *v = term.exp;
                }
            } else {
                harmony.insert(term.base, term.exp);
            }
        }
    }

    let mut r: u64 = 1;
    for (base, exp) in harmony.iter() {
        r *= base.pow(*exp) as u64
    }
    r
}

pub fn twelve_a<I>(buf: I) -> i32
where
    I: BufRead
{
    let bodies = read_locations(buf);
    sim(bodies, 1000)
}

pub fn twelve_b<I>(buf: I) -> u64
where
    I: BufRead
{
    let bodies = read_locations(buf);
    sim_cycle(bodies)
}


#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use std::iter::FromIterator;
    use super::*;

    #[test]
    fn test_read_locations() {
        let input = b"<x=-1, y=0, z=2>\n";
        let bodies = read_locations(&input[..]);
        assert_eq!(bodies.len(), 1);
        assert!(bodies[0].pos == [-1,0,2]);

        let input = b"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let bodies = read_locations(&input[..]);
        assert_eq!(bodies.len(), 4);
    }

    #[test]
    fn test_twelve_a() {
        let input = b"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let bodies = read_locations(&input[..]);
        assert_eq!(sim(bodies, 10), 179);
    }

    #[test]
    fn test_twelve_b() {
        let input = b"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let bodies = read_locations(&input[..]);
        assert_eq!(sim_cycle(bodies), 2772);
    }


    #[test]
    fn test_twelve_b_long() {
        let input = b"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
        let bodies = read_locations(&input[..]);
        assert_eq!(sim_cycle(bodies), 4686774924);
    }


    #[test]
    fn test_prime_fact() {
        let pf = prime_fact(2802);
        let pf_set: HashSet<_> = HashSet::from_iter(pf.iter().cloned());
        assert!(pf_set.contains(&Term{base: 2, exp: 1}));
        assert!(pf_set.contains(&Term{base: 3, exp: 1}));
        assert!(pf_set.contains(&Term{base: 467, exp: 1}));

        let pf = prime_fact(2028);
        let pf_set: HashSet<_> = HashSet::from_iter(pf.iter().cloned());
        assert!(pf_set.contains(&Term{base: 2, exp: 2}));
        assert!(pf_set.contains(&Term{base: 3, exp: 1}));
        assert!(pf_set.contains(&Term{base: 13, exp: 2}));
    }
}
