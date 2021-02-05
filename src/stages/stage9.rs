use std::io::prelude::*;
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

fn find_invalid_number(numbers: &Vec<usize>) -> anyhow::Result<usize> {
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

pub fn stage9_1() -> anyhow::Result<usize> {
    let numbers = get_numbers().context("Failed to parse numbers files")?;
    find_invalid_number(&numbers)
}

pub fn stage9_2() -> anyhow::Result<usize> {
    let numbers = get_numbers().context("Failed to parse numbers files")?;
    let target = find_invalid_number(&numbers)?;
    for i in 0..numbers.len() {
        let mut sum = 0;
        for j in i..numbers.len() {
            sum += numbers[j];
            if sum == target {
                return Ok(
                    numbers[i..j].iter().min().ok_or(anyhow::anyhow!("empty range"))? +
                    numbers[i..j].iter().max().ok_or(anyhow::anyhow!("empty range"))?
                )
            }
            if sum > target {
                break
            }
        }
    }
    anyhow::bail!("No weakness found");
}

