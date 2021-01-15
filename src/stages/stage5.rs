use std::io::prelude::*;
use nom::{
    IResult,
    bytes::complete::take_while_m_n,
    combinator::map_res,
    sequence::tuple
};

#[derive(Debug)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn seat_id(&self) -> usize {
        ((self.row as usize) << 3) + self.col as usize
    }
}

/// Parses a string as if it was a bit stream, with `zero` representing 0 and `one` 1.
fn bit_translate(zero: char, one: char, input: &str) -> anyhow::Result<usize> {
    if input.chars().count() > std::mem::size_of::<usize>() * 8 {
        anyhow::bail!("Too many bits for usize");
    }

    input.chars()
    .map(|c| match c {
        c if c == zero => Some(0),
        c if c == one => Some(1),
        _ => None
    })
    .try_fold(0usize, |acc, b| Some((acc << 1) + b?)).ok_or(anyhow::anyhow!("invalid chars"))
}

fn make_seat((row, col): (&str, &str)) -> anyhow::Result<Seat> {
    Ok(Seat {
        row: bit_translate('F', 'B', row)? as u8,
        col: bit_translate('L', 'R', col)? as u8,
    })
}

fn seat(input: &str) -> IResult<&str, Seat> {
    map_res(
        tuple((
            take_while_m_n(7, 7, |c| c == 'B' || c == 'F'),
            take_while_m_n(3, 3, |c| c == 'R' || c == 'L'),
        )),
        make_seat
    )(input)
  }

pub fn stage5_1() -> Option<usize> {
    let mut f = std::fs::File::open("stage5.txt").unwrap();
    let mut seats = String::new();

    f.read_to_string(&mut seats).unwrap();
    seats.split("\n")
    .filter_map(|s| seat(s).ok())
    .map(|(_, s)| s.seat_id())
    .max()
}

pub fn stage5_2() -> Vec<usize> {
    let mut f = std::fs::File::open("stage5.txt").unwrap();
    let mut seats = String::new();

    f.read_to_string(&mut seats).unwrap();
    let mut seats = seats.split("\n")
    .filter_map(|s| seat(s).ok())
    .map(|(_, s)| s.seat_id())
    .collect::<Vec<usize>>();
    seats.sort();

    // Find single empty seats
    (0..seats.len() - 1)
    .filter(|&i| seats[i] == seats[i + 1] - 2)
    .map(|i| seats[i] + 1)
    .collect()
}

