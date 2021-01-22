use std::{io::prelude::*, thread::current};
use std::collections::HashMap;
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::{opt, map_res},
    multi::separated_list1,
    sequence::{tuple, terminated}
};

#[derive(Debug)]
struct Rule {
    container: String,
    content: Vec<(String, usize)>,
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, container) = terminated(take_until(" bags contain "), tag(" bags contain "))(input)?;

    if input == "no other bags." {
        Ok((input, Rule {
            container: container.to_owned(),
            content: vec![],
        }))
    } else {
        let (input, content) = separated_list1(
            tag(", "),
            tuple((
                map_res(digit1, str::parse::<usize>),
                tag(" "),
                take_until(" bag"),
                tag(" bag"),
                opt(tag("s")),
            ))
        )(input)?;
        Ok((input, Rule {
            container: container.to_owned(),
            content: content.iter().map(|(num, _, color, _, _)| ((*color).to_owned(), *num)).collect(),
        }))
    }
}

fn get_rules() -> HashMap<String, Vec<(String, usize)>> {
    let mut f = std::fs::File::open("stage7.txt").unwrap();
    let mut rules = String::new();

    f.read_to_string(&mut rules).unwrap();
    rules.split("\n").filter_map(|r| rule(r).ok()).map(|(_, r)| (r.container, r.content)).collect()
}
pub fn stage7_1(owned_color: String) -> usize {
    let rules = get_rules();
    let mut count = 0;
    for (_, content) in rules.iter() {
        let mut current_colors: Vec<&str> = content.iter().map(|(c, _)| &c[..]).collect();
        let mut final_colors: Vec<&str> = Vec::new();
        if current_colors.iter().any(|&i| i == owned_color) {
            count += 1;
            continue;
        }

        loop {
            if let Some(c) = current_colors.pop() {
                if let Some(new_colors) = rules.get(c) {
                    if new_colors.iter().any(|(new_color, _)| *new_color == owned_color) {
                        count += 1;
                        break;
                    }
                    if new_colors.len() == 0 {
                        final_colors.push(c);
                    } else {
                        current_colors.extend(new_colors.iter().map(|(new_c, _)| &new_c[..]));
                    }
                }
            } else {
                break
            }
        }
    }
    return count;
}

pub fn stage7_2() -> () {
    // let mut f = std::fs::File::open("stage6.txt").unwrap();
    // let mut answers = String::new();

    // f.read_to_string(&mut answers).unwrap();
    // answers.split("\n\n")
    // .map(|g| g
    //     // make a hashset out of each person's answers
    //     .split("\n")
    //     .filter(|a| a.chars().count() != 0) // remove accidental empty lines
    //     .map(|a| a.chars().collect::<std::collections::HashSet<char>>())
    //     .fold( // intersect all answers of this group
    //         ('a'..='z').collect::<std::collections::HashSet<char>>(),
    //         |acc, a| acc.intersection(&a).cloned().collect())
    //     .len())
    // .sum()
}
