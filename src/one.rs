use std::io::BufRead;

fn fuel_for_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn one_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut total: i32 = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        total += fuel_for_mass(line.parse::<i32>().expect("error parsing input line as number"));
    }
    total
}

pub fn one_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut total: i32 = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        let mut fuel: i32 = fuel_for_mass(
            line.parse::<i32>().expect("error parsing input line as number")
        );
        while fuel > 0 {
            total += fuel;
            fuel = fuel_for_mass(fuel);
        }
    }
    total
}
