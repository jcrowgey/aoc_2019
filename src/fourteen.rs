use std::io::BufRead;
use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;


#[derive(Debug)]
struct Rule {
    amt: usize,
    lhs: HashMap<String,usize>,
}

fn parse_amtmat(pair_str: String) -> (String,usize) {
    let pair: Vec<_> = pair_str.trim().split(" ").collect();
    (
        pair[1].to_string(),
        pair[0].parse::<usize>().expect("could not parse amount"),
    )
}

fn parse_lhs(lhs_str: String) -> HashMap<String,usize>
{
    let mut lhs = HashMap::new();
    let pairs = lhs_str.trim().split(", ");
    for pair in pairs {
        let (mat, amt) = parse_amtmat(pair.to_string());
        lhs.insert(mat, amt);
    }
    lhs
}

fn read_rules<I>(buf: I) -> HashMap<String,Rule>
where
    I: BufRead,
{
    let mut rules = HashMap::new();
    for line in buf.lines() {
        let line = line.unwrap();
        let rule: Vec<_> = line.trim().split("=>").collect();
        let (rhs_mat, rhs_amt) = parse_amtmat(rule[1].to_string());
        rules.insert(
            rhs_mat,
            Rule{
                amt: rhs_amt,
                lhs: parse_lhs(rule[0].to_string()),
            }
        );
    }
    rules
}

fn rule_fires(req_amt: usize, amt_produced: usize) -> usize {
    let mut res = req_amt / amt_produced;
    if req_amt % amt_produced > 0 {  // once more for the remainder
        res += 1;
    }
    res
}

// return a partial ordering of the materials
fn ord(rules: &HashMap<String,Rule>) -> Vec<HashSet<String>> {
    let mut res = Vec::new();
    let tier_0 = HashSet::from_iter(
        rules.get("FUEL").unwrap().lhs.keys().cloned::<String>(),
    );
    let mut tier = 0;
    res.push(tier_0);
    loop {
        let this_tier: HashSet<String> = res[tier].to_owned();
        let mut next_tier = HashSet::new();
        for material in this_tier.iter() {
            if material == "ORE" {
                continue;
            }
            for req in rules.get(material).unwrap().lhs.keys() {
                // remove from the current or previous tiers!
                for i in 0 .. tier + 1 {
                    res[i].remove(req);
                }
                next_tier.insert(req.to_owned());
            }
        }
        if next_tier.len() == 1 && next_tier.contains("ORE") {
            break;
        }
        res.push(next_tier);
        tier += 1;

    }
    res
}

// shove numbers into the deps and return ore
fn counted_deps(rules: &HashMap<String,Rule>, deps: &Vec<HashSet<String>>, fuel_amt: usize) -> HashMap<String,usize> {
    let mut counted_deps = HashMap::new(); // rules.get("FUEL").unwrap().lhs.to_owned();  // seed with FUEL reqs
    for (mat, amt) in &rules.get("FUEL").unwrap().lhs.to_owned() {
        counted_deps.insert(mat.to_string(), amt * fuel_amt);
    }

    for tier in deps {
        for material in tier {
            let req_amt = *counted_deps.get(material).unwrap();
            let mat_rule = rules.get(material).unwrap();
            let rule_n = rule_fires(req_amt, mat_rule.amt);
            for (dep_mat, dep_amt) in &mat_rule.lhs {
                let insert_amt = dep_amt * rule_n;
                if let Some(cd) = counted_deps.get_mut(dep_mat) {
                    *cd += insert_amt;
                } else {
                    counted_deps.insert(dep_mat.to_string(), insert_amt);
                }
            }
        }
    }
    counted_deps
}

/*
fn purchase(material: &String, amt: usize, rule: &Rule, bank: &mut HashMap<String, usize>) {
    // pay the piper!
    let mut purchased = 0;
    while purchased < amt {
        for (req_mat, to_pay) in &rule.lhs {
            let banked_amt = bank.get_mut(&req_mat.to_string()).unwrap();
            *banked_amt -= to_pay;
        }
        purchased += rule.amt;
    }

    if bank.contains_key(material) {
        *bank.get_mut(material).unwrap() += purchased;
    } else {
        bank.insert(material.to_string(), purchased);
    }
}

fn make_one_fuel(
    rules: &HashMap<String,Rule>,
    deps: &Vec<HashSet<String>>,
    counted: &HashMap<String,usize>,
    mut bank: &mut HashMap::<String,usize>,
) {

    for tier in deps.iter().rev() {
        for material in tier {
            let mat_rule = rules.get(material).unwrap();
            let have = *bank.get(material).unwrap_or(&0);
            let req_amt = *counted.get(material).unwrap();
            if have < req_amt {
                let need = req_amt - have;
                purchase(material, need, &mat_rule, &mut bank);
            }
        }
    }
    let fuel_rule = rules.get("FUEL").unwrap();
    purchase(&"FUEL".to_string(), 1, &fuel_rule, &mut bank);
}
*/

pub fn fourteen_a<I>(buf: I) -> usize
where
    I: BufRead,
{
    let rules = read_rules(buf);
    let deps = ord(&rules);
    *counted_deps(&rules, &deps, 1).get("ORE").unwrap()
}

pub fn fourteen_b<I>(buf: I) -> usize
where
    I: BufRead,
{
    let rules = read_rules(buf);
    let deps = ord(&rules);
    let cost_for_one = *counted_deps(&rules, &deps, 1).get("ORE").unwrap();
    let ore_supply: usize = 1000000000000;

    // start with counted(1).ORE / supply
    let mut guess = ore_supply / cost_for_one;
    let mut guess_counted = counted_deps(&rules, &deps, guess);
    let mut ore_cost = *guess_counted.get("ORE").unwrap();
    loop {
        let remainder = ore_supply - ore_cost;
        let guess_incr = remainder / cost_for_one;
        guess += guess_incr;
        guess_counted = counted_deps(&rules, &deps, guess);
        let next_cost = *guess_counted.get("ORE").unwrap();
        if next_cost == ore_cost {
            break;
        }
        ore_cost = next_cost;
    }
    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fourteen_a() {
        let input = b"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        assert_eq!(fourteen_a(&input[..]), 31);

        let input = b"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        assert_eq!(fourteen_a(&input[..]), 165);
    }

    #[test]
    fn test_fourteen_big1() {
        let input = b"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(fourteen_a(&input[..]), 13312);
        assert_eq!(fourteen_b(&input[..]), 82892753);
    }

    #[test]
    fn test_fourteen_a_shared_middle() {
        let input = b"1 ORE => 2 A
1 ORE => 1 B
1 A, 1 B => 2 C
1 A, 1 C => 1 D
1 C, 1 D => 1 FUEL";
        assert_eq!(fourteen_a(&input[..]), 2);
    }

    #[test]
    fn test_fourteen_big2() {
        let input = b"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        assert_eq!(fourteen_a(&input[..]), 180697);
        assert_eq!(fourteen_b(&input[..]), 5586022);
    }

    #[test]
    fn test_fourteen_big3() {
        let input = b"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        assert_eq!(fourteen_a(&input[..]), 2210736);
        assert_eq!(fourteen_b(&input[..]), 460664);
    }
}
