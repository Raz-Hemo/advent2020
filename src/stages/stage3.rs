use std::io::prelude::*;

pub fn stage3_1(xslope:usize, yslope: usize) ->usize {
    let mut f = std::fs::File::open("stage3.txt").unwrap();
    let mut map = String::new();

    f.read_to_string(&mut map).unwrap();
    map.split('\n')
    .enumerate()
    .filter(|(i, v)| i % yslope == 0 && v.len() != 0)
    .filter(|(i, v)| v.chars().nth((i * xslope / yslope) % v.len()).unwrap() == '#')
    .count()
}

pub fn stage3_2() {
    println!("product of trees: {}",
        vec![(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(xs, ys)| stage3_1(xs, ys))
        .fold(1, |acc, x| acc * x)
    );
}
