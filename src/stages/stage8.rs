use std::io::prelude::*;
use std::str::FromStr;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{map_res, rest},
    sequence::tuple
};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct Instruction {
    ins: InsType,
    arg: isize,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (ins, _, arg)) = tuple((
        map_res(alpha1, |i: &str| i.parse::<InsType>()),
        tag(" "),
        map_res(rest, |i: &str| i.parse::<isize>())
    ))(input)?;
    Ok((input, Instruction {
        ins,
        arg,
    }))
}

fn get_code() -> anyhow::Result<Vec<Instruction>> {
    let mut f = std::fs::File::open("stage8.txt").unwrap();
    let mut instructions = String::new();

    f.read_to_string(&mut instructions).unwrap();
    Ok(
        instructions.split("\n")
        .filter(|i| *i != "")
        .map(|i| 
            instruction(i).map(|(_, i)| i)
            .map_err(|e| anyhow::anyhow!("invalid instruction: {}", e))
        )
        .collect::<Result<Vec<Instruction>, _>>()?
    )
}

fn checked_index_add(n: usize, m: isize) -> anyhow::Result<usize> {
    if m < 0 {
        n.checked_sub(m.wrapping_abs() as usize)
    } else {
        n.checked_add(m as usize)
    }.ok_or(anyhow::anyhow!("overflow"))
}

fn run_vm(instructions: &Vec<Instruction>) -> anyhow::Result<(isize, bool)> {
    let mut executed_addrs = std::collections::HashSet::new();
    let mut ip: usize = 0;
    let mut acc: isize = 0;
    loop {
        if executed_addrs.get(&ip).is_some() || ip == instructions.len() {
            break Ok((acc, ip == instructions.len()))
        }
        match instructions.get(ip) {
            Some(Instruction {ins: InsType::Nop, ..}) => (),
            Some(Instruction {ins: InsType::Acc, arg}) => {acc += *arg;},
            Some(Instruction {ins: InsType::Jmp, arg}) => {ip = checked_index_add(ip, *arg - 1)?;},
            None => anyhow::bail!("invalid ip {}", ip),
        }
        executed_addrs.insert(ip);
        ip += 1;
    }
}

pub fn stage8_1() -> anyhow::Result<isize> {
    let instructions = get_code()?;
    let (acc, _) = run_vm(&instructions)?;
    Ok(acc)
}

pub fn stage8_2() -> anyhow::Result<isize> {
    let mut instructions = get_code()?;
    let flip_instruction = |i| match i {
        Instruction {ins: InsType::Nop, arg} => Instruction {ins: InsType::Jmp, arg},
        Instruction {ins: InsType::Jmp, arg} => Instruction {ins: InsType::Nop, arg},
        i => i
    };

    for i in 0..instructions.len() {
        instructions[i] = flip_instruction(instructions[i]);
        let (acc, ended_normally) = run_vm(&instructions)?;
        if ended_normally {
            return Ok(acc)
        }
        instructions[i] = flip_instruction(instructions[i]);
    }
    anyhow::bail!("No flip results in a valid program")
}
