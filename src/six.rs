use std::io::BufRead;
use std::collections::{HashMap,HashSet};

pub fn six_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let direct_orbits = lines_to_orbits(buf);
    total_orbits(direct_orbits)
}


pub fn six_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    let direct_orbits = lines_to_orbits(buf);
    min_transfers(direct_orbits)
}

fn lines_to_orbits<I>(buf: I) -> Vec<[String; 2]>
where
    I: BufRead,
{
    let mut direct_orbits = Vec::new();
    for line in buf.lines() {
        let direct_orbit: String = line.unwrap();
        let direct_orbit: Vec<String> = direct_orbit.split(")")
                                                    .map(|s| s.to_string())
                                                    .collect();
        let direct_orbit: [String; 2] = [
            direct_orbit[0].to_owned(),
            direct_orbit[1].to_owned()
        ];
        direct_orbits.push(direct_orbit);
    }
    direct_orbits
}

// I think this problem amounts to either a convergence approach or a 
// sorting approach.  I'm doing convergence since sorting implies multiple
// passes anyway.
fn total_orbits(direct_orbits: Vec<[String; 2]>) -> i32 {

    let mut is_orbited = HashMap::new();
    let mut completed = HashSet::new();

    while completed.len() < direct_orbits.len() {
        for [focus, mass] in direct_orbits.iter() {
            if focus == &"COM" {
                is_orbited.insert(mass, 1);
                completed.insert([focus, mass]);
            }
            else if is_orbited.contains_key(focus) {
                let f = is_orbited.get(focus).unwrap().to_owned();
                is_orbited.insert(mass, f+1);
                completed.insert([focus, mass]);
            }
        }
    }
    is_orbited.values().sum()
}

// builds a hash map of the ancestors of YOU and SAN to their generation walks up the YOU line till
// it finds a common ancestor.  Since this is a tree, that's the shortest path.
fn min_transfers(direct_orbits: Vec<[String; 2]>) -> i32 {

    let mut orbits = HashMap::new();
    for [focus, mass] in direct_orbits.iter() {
        orbits.insert(mass.to_string(), focus.to_string());
    }


    let you_ancestors = calc_ancestors(&(orbits), "YOU".to_string());
    let san_ancestors = calc_ancestors(&(orbits), "SAN".to_string());

    let mut sorted_ancestors: Vec<_> = you_ancestors.iter().collect();
    sorted_ancestors.sort_by(|a, b| b.1.cmp(a.1).reverse());
    for (ann, dist) in sorted_ancestors.iter() {
        if san_ancestors.contains_key(*ann) {
            let san_dist = san_ancestors.get(*ann).unwrap().to_owned();
            return *dist + san_dist
        }
    }
    return -1
}

fn calc_ancestors(orbit_map: &HashMap<String, String>, object: String) -> HashMap<String, i32> {
    let mut ancestors = HashMap::new();
    let mut curr = object;
    let mut ann = orbit_map.get(&curr).unwrap().to_owned().to_string();
    let mut i = 0;
    loop {
        ancestors.insert(ann.to_owned(), i);
        curr = ann;
        ann = orbit_map.get(&curr).unwrap().to_owned().to_string();
        i += 1;
        if ann == "COM" {
            break;
        }
    }
    ancestors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_six_a() {
        let input = b"COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(six_a(&input[..]), 42);
    }

    #[test]
    fn test_six_b() {
        let input = b"COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(six_b(&input[..]), 4);
    }
}
