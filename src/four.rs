use std::io::BufRead;
use std::ops::Range;

// This is a brute-force approach.  A function based on encoding rules such that for a given 6 char
// password, we generate the next one in sequence would be more elegant, I think.  But that
// generator function is not immediately obvious to me, while this brute-force validation approach
// is.
//
// This is also why I'm using a numeric array here, because I had imagined doing the `next_valid`
// function and wanted something incr-able.

// "i_to_array": get it?
fn itoa(i: i32) -> [u8; 6] {
    [(i % 1000000 / 100000) as u8,
     (i % 100000 / 10000) as u8,
     (i % 10000 / 1000) as u8,
     (i % 1000 / 100) as u8,
     (i % 100 / 10) as u8,
     (i % 10) as u8,
    ]
}

fn is_valid_a(pwd: &[u8; 6]) -> bool {
    is_incr(pwd) && has_repeat(pwd)
}

fn is_valid_b(pwd: &[u8; 6]) -> bool {
    is_incr(pwd) && has_repeat_wo_trips(pwd)
}

fn is_incr(pwd: &[u8; 6]) -> bool {
    for (i, c) in pwd.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        if pwd[i-1] > *c {
            return false;
        }
    }
    true
}

fn has_repeat_wo_trips(pwd: &[u8; 6]) -> bool {
    let mut cur_seq: u8 = pwd[0];
    let mut len_seq = 1;
    let mut ok = false;
    for (i, c) in pwd.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        if *c == cur_seq {
            len_seq += 1;
        } else {
            cur_seq = *c;
            len_seq = 1;
        }
        if len_seq == 2 {
            ok = true;
        } else if len_seq == 3 {
            ok = false;
        }
        if ok == true && len_seq == 1 {
            return ok
        }
    }
    ok
}

fn has_repeat(pwd: &[u8; 6]) -> bool {
    let mut cur_seq: u8 = pwd[0];
    let mut len_seq = 1;
    let mut ok = false;
    for (i, c) in pwd.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        if *c == cur_seq {
            len_seq += 1;
        } else {
            cur_seq = *c;
            len_seq = 1;
        }
        if len_seq == 2 {
            ok = true;
        }
    }
    ok
}

fn read_range<I>(mut buf: I) -> Range<i32>
where
    I: BufRead
{
    let mut line = String::new();
    buf.read_line(&mut line).unwrap();
    let lh: Vec<i32> = line.trim()
        .split("-")
        .into_iter()
        .map(|x| x.parse().expect("error parsing number"))
        .collect();

    let rng: Range<i32> = lh[0]..lh[1] + 1; // range is not inclusive on the high 
    rng

}

pub fn four_a<I>(buf: I) -> usize
where
    I: BufRead
{
    let rng = read_range(buf);
    let valid: Vec<i32> = rng.filter(|n| is_valid_a(&itoa(*n))).collect();
    valid.len()
}

pub fn four_b<I>(buf: I) -> usize
where
    I: BufRead
{
    let rng = read_range(buf);
    let valid: Vec<i32> = rng.filter(|n| is_valid_b(&itoa(*n))).collect();
    valid.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_repeat_wo_trips() {
        assert_eq!(has_repeat_wo_trips(&[1,1,2,2,3,3]), true);
        assert_eq!(has_repeat_wo_trips(&[1,2,3,4,4,4]), false);
        assert_eq!(has_repeat_wo_trips(&[1,1,1,1,2,2]), true);
        assert_eq!(has_repeat_wo_trips(&[1,1,1,1,1,1]), false);
        assert_eq!(has_repeat_wo_trips(&[1,1,2,2,2,2]), true);
    }

    #[test]
    fn test_is_valid_a() {
        assert_eq!(is_valid_a(&[1,1,1,1,1,1]), true);
        assert_eq!(is_valid_a(&[2,2,3,4,5,0]), false);
        assert_eq!(is_valid_a(&[1,2,3,7,8,9]), false);
    }
}
