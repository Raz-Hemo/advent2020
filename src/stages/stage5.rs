use std::io::prelude::*;

pub fn stage5_1() -> usize {
    let mut f = std::fs::File::open("stage5.txt").unwrap();
    let mut seats = String::new();

    f.read_to_string(&mut seats).unwrap();
    seats.split("\n")
    // .map(|p| passport(p))
    // .filter_map(Result::ok)
    // .filter(|(_, p)| p.byr.is_some() && p.iyr.is_some() && p.eyr.is_some() && 
    //                  p.hgt.is_some() && p.hcl.is_some() && p.ecl.is_some() && p.pid.is_some())
    .count()
}
