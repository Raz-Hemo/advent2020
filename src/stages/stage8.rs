use std::io::prelude::*;
use std::str::FromStr;
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, alpha1},
    combinator::{opt, map_res},
    multi::separated_list1,
    branch::alt,
    sequence::tuple
};

enum InsType {
    Nop,
    Acc,
    Jmp
}
impl FromStr for InsType {
    type Err = String;
    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match i {
            "nop" => Ok(InsType::Nop),
            "acc" => Ok(InsType::Acc),
            "jmp" => Ok(InsType::Jmp),
            _ => Err(format!("Invalid instruction {}", i)),
        }
    }
}

struct Instruction {
    ins: InsType,
    arg: isize,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (ins, _)) = tuple((map_res(alpha1, |i: &str| i.parse::<InsType>()), tag(" ")))(input)?;
    Ok((input, Instruction {
        ins,
        arg: input.parse()?,
    }))
}

fn get_code() -> Vec<Instruction> {
    let mut f = std::fs::File::open("stage7.txt").unwrap();
    let mut instructions = String::new();

    f.read_to_string(&mut instructions).unwrap();
    instructions.split("\n").filter_map(|i| instruction(i).ok()).map(|(_, i)| i).collect()
}

pub fn stage8_1(owned_color: String) -> usize {
    let instructions = get_code();
}

pub fn stage8_2(owned_color: String) -> usize {

}
