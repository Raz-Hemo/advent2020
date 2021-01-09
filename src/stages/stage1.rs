use std::io::prelude::*;

pub fn stage1_1() {
    let mut f = std::fs::File::open("stage1.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).unwrap();
    let data: Vec<usize> = data.split('\n').map(|s| s.parse()).filter_map(Result::ok).collect();
    
    for i in 0..data.len() {
        for j in i..data.len() {
            if data[i] + data[j] == 2020 {
                println!("found: {}", data[i] * data[j]);
            }
        }
    }
}

pub fn stage1_2() {
    let mut f = std::fs::File::open("stage2.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).unwrap();
    let data: Vec<usize> = data.split('\n').map(|s| s.parse()).filter_map(Result::ok).collect();
    
    for i in 0..data.len() {
        for j in i..data.len() {
            for k in j..data.len() {
                if data[i] + data[j] + data[k] == 2020 {
                    println!("found: {}", data[i] * data[j] * data[k]);
                }
            }
        }
    }
}
