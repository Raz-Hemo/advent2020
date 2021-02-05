use std::io::prelude::*;
use anyhow::Context;
use std::collections::HashMap;

fn get_adapters() -> anyhow::Result<Vec<usize>> {
    let mut f = std::fs::File::open("stage10.txt").unwrap();
    let mut adapters = String::new();

    f.read_to_string(&mut adapters).unwrap();
    let mut adapters = adapters.split("\n")
        .filter(|n| *n != "")
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    // add the port and device, which have 0 and MAX+3 ratings
    adapters.push(0);
    if let Some(&max) = adapters.iter().max() {
        adapters.push(max + 3);
    }

    // Sort into order and get rid of mut
    adapters.sort();
    Ok(adapters)
}

pub fn stage10_1() -> anyhow::Result<usize> {
    let mut adapters = get_adapters().context("Failed to parse adapter list")?;

    let mut differences: HashMap<usize, usize> = HashMap::new();
    for i in 1..adapters.len() {
        *differences.entry(adapters[i] - adapters[i - 1]).or_insert(0) += 1;
    }
    println!("{:?}", differences);
    println!("{:?}", adapters);
    Ok(differences.get(&1).unwrap_or(&0) * differences.get(&3).unwrap_or(&0))
}

pub fn stage10_2() -> anyhow::Result<usize> {
    let adapters = get_adapters().context("Failed to parse adapter list")?;
    let mut ways_to_reach = vec![
        0; *adapters.iter().max().ok_or(anyhow::anyhow!("Empty adapter list"))? + 1
    ];
    ways_to_reach[0] = 1;

    for i in 1..ways_to_reach.len() {
        if !adapters.contains(&i) { continue }

        ways_to_reach[i] += ways_to_reach[i - 1];
        if i > 1 {
            ways_to_reach[i] += ways_to_reach[i - 2]
        }
        if i > 2 {
            ways_to_reach[i] += ways_to_reach[i - 3]
        }
    }

    ways_to_reach.last().ok_or(anyhow::anyhow!("Empty path vector")).map(|x| *x)
}
