use std::io::prelude::*;
use std::str::FromStr;
use anyhow::Context;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Seat {
    Floor,
    Taken,
    Empty
}
impl FromStr for Seat {
    type Err = anyhow::Error;
    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match i {
            "L" => Ok(Seat::Empty),
            "." => Ok(Seat::Floor),
            "#" => Ok(Seat::Taken),
            _ => Err(anyhow::anyhow!("Invalid seat {}", i)),
        }
    }
}


fn get_seats() -> anyhow::Result<Vec<Vec<Seat>>> {
    let mut f = std::fs::File::open("stage11.txt").unwrap();
    let mut seats = String::new();

    f.read_to_string(&mut seats).unwrap();
    seats.split("\n")
    .filter(|n| *n != "")
    .map(|n|
        n.chars()
        .map(|c| c.to_string().parse::<Seat>())
        .collect::<Result<Vec<Seat>, _>>()
    )
    .collect::<Result<Vec<Vec<Seat>>, _>>()
}

fn run_step_stage1(seats: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut new_seats = seats.clone();
    let offsets = [(-1i16, -1i16), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for row in 0..seats.len() {
        for col in 0..seats[row].len() {
            let taken_count = 
                offsets.iter()
                .map(|o| 
                    *seats.get((row as i16 + o.0) as usize)
                    .unwrap_or(&vec![])
                    .get((col as i16 + o.1) as usize).unwrap_or(&Seat::Floor)
                ).filter(|&s| s == Seat::Taken).count();

            if seats[row][col] == Seat::Empty && taken_count == 0 {
                new_seats[row][col] = Seat::Taken;
            } else if seats[row][col] == Seat::Taken && taken_count >= 4 {
                new_seats[row][col] = Seat::Empty;
            }
        }
    }
    new_seats
}

fn run_step_stage2(seats: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut new_seats = seats.clone();
    let offsets = [(-1i16, -1i16), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for row in 0..seats.len() {
        for col in 0..seats[row].len() {
            let taken_count = 
                offsets.iter()
                .filter(|&&(ox, oy)| {
                    let mut curx = ox + col as i16;
                    let mut cury = oy + row as i16;
                    // we assume that all rows and cols are the same dimensions
                    while curx >= 0 && curx < seats[row].len() as i16 && cury >= 0 && cury < seats.len() as i16 {
                        match seats[cury as usize][curx as usize] {
                            Seat::Taken => { return true }
                            Seat::Empty => { return false }
                            _ => { curx += ox; cury += oy; }
                        } 
                    }
                    return false;
                })
                .count();

            if seats[row][col] == Seat::Empty && taken_count == 0 {
                new_seats[row][col] = Seat::Taken;
            } else if seats[row][col] == Seat::Taken && taken_count >= 5 {
                new_seats[row][col] = Seat::Empty;
            }
        }
    }
    new_seats
}

fn simulate(step_function: &dyn Fn(&Vec<Vec<Seat>>) -> Vec<Vec<Seat>>) -> anyhow::Result<usize> {
    let mut last_step = get_seats().context("Failed to parse seat map")?;
    let mut current_step = step_function(&last_step);

    while last_step != current_step {
        last_step = current_step.clone();
        current_step = step_function(&current_step);
    }
    Ok(current_step.iter().map(|r| r.iter().filter(|&&s| s == Seat::Taken).count()).sum())
}

pub fn stage11_1() -> anyhow::Result<usize> {
    simulate(&run_step_stage1)
}

pub fn stage11_2() -> anyhow::Result<usize> {
    simulate(&run_step_stage2)
}
