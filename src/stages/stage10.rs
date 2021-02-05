use std::io::prelude::*;
use anyhow::Context;
use std::collections::HashMap;

fn get_adapters() -> anyhow::Result<Vec<usize>> {
    let mut f = std::fs::File::open("stage10.txt").unwrap();
    let mut adapters = String::new();

    f.read_to_string(&mut adapters).unwrap();
    Ok(
        adapters.split("\n")
        .filter(|n| *n != "")
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?
    )
}

pub fn stage10_1() -> anyhow::Result<usize> {
    let mut adapters = get_adapters().context("Failed to parse adapter list")?;

    // add the port and device, which have 0 and MAX+3 ratings
    adapters.push(0);
    if let Some(&max) = adapters.iter().max() {
        adapters.push(max + 3);
    }

    // Sort into order and get rid of mut
    adapters.sort();
    let adapters = adapters;

    let mut differences: HashMap<usize, usize> = HashMap::new();
    for i in 1..adapters.len() {
        *differences.entry(adapters[i] - adapters[i - 1]).or_insert(0) += 1;
    }
    println!("{:?}", differences);
    println!("{:?}", adapters);
    Ok(differences.get(&1).unwrap_or(&0) * differences.get(&3).unwrap_or(&0))
}
