use std::io::prelude::*;
use std::str::FromStr;
use anyhow::Context;

fn get_numbers() -> anyhow::Result<Vec<usize>> {
    let mut f = std::fs::File::open("stage9.txt").unwrap();
    let mut numbers = String::new();

    f.read_to_string(&mut numbers).unwrap();
    Ok(
        numbers.split("\n")
        .filter(|n| *n != "")
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?
    )
}

pub fn stage9_1() -> anyhow::Result<usize> {
    let numbers = get_numbers().context("Failed to parse numbers files")?;
    'outer: for i in 25..numbers.len() {
        for j in (i-25)..i {
            for k in (j+1)..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    continue 'outer
                }
            }
        }
        return Ok(numbers[i])
    }
    anyhow::bail!("No invalid number found")
}

pub fn stage9_2() -> () {

}

