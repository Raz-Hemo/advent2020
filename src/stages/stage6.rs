use std::io::prelude::*;


pub fn stage6_1() -> usize {
    let mut f = std::fs::File::open("stage6.txt").unwrap();
    let mut questions = String::new();

    f.read_to_string(&mut questions).unwrap();
    questions.split("\n\n")
    .map(|g| g.chars().filter(|&c| c != '\n').collect::<std::collections::HashSet<char>>())
    .map(|s| s.len())
    .sum()
}

pub fn stage6_2() -> usize {
    let mut f = std::fs::File::open("stage6.txt").unwrap();
    let mut answers = String::new();

    f.read_to_string(&mut answers).unwrap();
    answers.split("\n\n")
    .map(|g| g
        // make a hashset out of each person's answers
        .split("\n")
        .filter(|a| a.chars().count() != 0) // remove accidental empty lines
        .map(|a| a.chars().collect::<std::collections::HashSet<char>>())
        .fold( // intersect all answers of this group
            ('a'..='z').collect::<std::collections::HashSet<char>>(),
            |acc, a| acc.intersection(&a).cloned().collect())
        .len())
    .sum()
}
