use std::io::prelude::*;
use nom::{
    IResult,
    character::complete::{digit1, one_of},
    combinator::map_res,
    sequence::tuple
};


enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (ins, arg)) = tuple((
        one_of("NSEWRLF"),
        map_res(digit1, |i: &str| i.parse::<usize>())
    ))(input)?;
    Ok((input, match ins {
        'N' => Instruction::North(arg),
        'S' => Instruction::South(arg),
        'E' => Instruction::East(arg),
        'W' => Instruction::West(arg),
        'R' => Instruction::Right(arg),
        'L' => Instruction::Left(arg),
        'F' => Instruction::Forward(arg),
        _ => panic!("Invalid instruction {}", ins)
    }))
}


fn get_instructions() -> anyhow::Result<Vec<Instruction>> {
    let mut f = std::fs::File::open("stage12.txt").unwrap();
    let mut instructions = String::new();

    f.read_to_string(&mut instructions).unwrap();
    Ok(instructions.split("\n")
    .filter(|n| *n != "")
    .map(|n| instruction(n).map(|(_, ins)| ins).map_err(|e| anyhow::anyhow!("{:?}", e)))
    .collect::<Result<Vec<Instruction>, _>>()?)
}

pub fn stage12_1() -> anyhow::Result<isize> {
    let instrs = get_instructions()?;
    let mut x = 0isize;
    let mut y = 0isize;
    let mut direction = 0;

    for i in instrs {
        x += match i {
            Instruction::East(n) => n as isize,
            Instruction::West(n) => -(n as isize),
            Instruction::Forward(n) => ((direction as f32).to_radians().cos() * n as f32) as isize,
            _ => 0
        };
        y += match i {
            Instruction::South(n) => -(n as isize),
            Instruction::North(n) => n as isize,
            Instruction::Forward(n) => ((direction as f32).to_radians().sin() * n as f32) as isize,
            _ => 0
        };
        direction = direction + (match i {
            Instruction::Left(n) => n as isize,
            Instruction::Right(n) => -(n as isize),
            _ => 0
        }) % 360;
        println!("{} {} {}", x, y, direction);
    }

    Ok(x.abs() + y.abs())
}

pub fn stage12_2() -> anyhow::Result<isize> {
    let instrs = get_instructions()?;
    let mut wx = 10isize;
    let mut wy = 1isize;
    let mut sx = 0isize;
    let mut sy = 0isize;

    for i in instrs {
        println!("ship: ({}, {}), wp: ({}, {})", sx, sy, wx, wy);
        match i {
            Instruction::East(n) => wx += n as isize,
            Instruction::West(n) => wx -= n as isize,
            Instruction::South(n) => wy -= n as isize,
            Instruction::North(n) => wy += n as isize,
            Instruction::Forward(n) => {
                sx += wx * n as isize;
                sy += wy * n as isize
            },
            Instruction::Left(n) => {
                let new_wx = (wx as f32 * ((n as f32).to_radians()).cos() - wy as f32 * ((n as f32).to_radians()).sin()).round() as isize;
                wy = (wy as f32 * ((n as f32).to_radians()).cos() + wx as f32 * ((n as f32).to_radians()).sin()).round() as isize;
                wx = new_wx;
            },
            Instruction::Right(n) => {
                let new_wx = (wx as f32 * (-(n as f32).to_radians()).cos() - wy as f32 * (-(n as f32).to_radians()).sin()).round() as isize;
                wy = (wy as f32 * (-(n as f32).to_radians()).cos() + wx as f32 * (-(n as f32).to_radians()).sin()).round() as isize;
                wx = new_wx;
            },
        };
    }

    Ok(sx.abs() + sy.abs())
}
