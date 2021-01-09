use std::io::prelude::*;

pub fn stage2_1() {
    let mut f = std::fs::File::open("stage2.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).unwrap();

    let re = regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    
    println!(
        "total: {}",
        re.captures_iter(&data)
        .filter(|v| {
            let count = v[4].matches(&v[3]).count();
            count >= v[1].parse().unwrap() && count <= v[2].parse().unwrap()
        })
        .count()
    );
}

pub fn stage2_2() {
    let mut f = std::fs::File::open("stage2.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).unwrap();

    let re = regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    
    println!(
        "total: {}",
        re.captures_iter(&data)
        .filter(|v| {
            (v[4].chars().nth(v[1].parse::<usize>().unwrap() - 1).unwrap() == v[3].chars().nth(0).unwrap()) ^
            (v[4].chars().nth(v[2].parse::<usize>().unwrap() - 1).unwrap() == v[3].chars().nth(0).unwrap())
        })
        .count()
    );
}
