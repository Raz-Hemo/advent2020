use std::{io::prelude::*, str::FromStr};
use std::collections::HashMap;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, is_not, is_a, take, take_while_m_n},
    sequence::{tuple, preceded},
    combinator::{map_res, opt},
};

#[derive(Debug, Clone, Copy)]
enum HeightType {
    Centimeters,
    Inches
}
struct ParseHeightTypeError {}
impl FromStr for HeightType {
    type Err = ParseHeightTypeError;
    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match i {
            "cm" => Ok(HeightType::Centimeters),
            "in" => Ok(HeightType::Inches),
            _ => Err(ParseHeightTypeError {}),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Height {
    typ: HeightType,
    height: u8,
}
fn height(input: &str) -> IResult<&str, Height> {
    let (input, (value, typ)) = tuple((
        map_res(take_while_m_n(2, 3, |c: char| c.is_digit(10)), u8::from_str),
        map_res(take(2usize), HeightType::from_str),
    ))(input)?;

    Ok((input, Height {
        height: value,
        typ: typ
    }))
}

#[derive(Debug, Clone, Copy)]
struct RgbColor {
    r: u8,
    g: u8,
    b: u8
}
struct ParseRgbColorError {}
impl FromStr for RgbColor {
    type Err = ParseRgbColorError;
    fn from_str(i: &str) -> Result<Self, Self::Err> {
        Ok(RgbColor {
            r: u8::from_str_radix(&i[0..2], 16).map_err(|_e| ParseRgbColorError {})?,
            g: u8::from_str_radix(&i[2..4], 16).map_err(|_e| ParseRgbColorError {})?,
            b: u8::from_str_radix(&i[4..6], 16).map_err(|_e| ParseRgbColorError {})?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum EyeColor {Amb,Blu,Brn,Gry,Grn,Hzl,Oth}
struct ParseEyeColorError {}
impl FromStr for EyeColor {
    type Err = ParseEyeColorError;
    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match i {
            "amb" => Ok(EyeColor::Amb),
            "blu" => Ok(EyeColor::Blu),
            "brn" => Ok(EyeColor::Brn),
            "gry" => Ok(EyeColor::Gry),
            "grn" => Ok(EyeColor::Grn),
            "hzl" => Ok(EyeColor::Hzl),
            "oth" => Ok(EyeColor::Oth),
            _ => Err(ParseEyeColorError {}),
        }
    }
}
named!(eye_color<EyeColor>, flat_map!(
    alt!(tag!("amb") | tag!("blu") | tag!("brn") | tag!("gry") | tag!("grn") | tag!("hzl") | tag!("oth")),
    parse_to!(EyeColor)
));



fn rgb_color(input: &str) -> IResult<&str, RgbColor> {
    preceded(
        tag("#"),
        map_res(
            take_while_m_n(6, 6, |c: char| c.is_digit(16)),
            RgbColor::from_str)
    )(input)
}

fn passport_tag(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (key, _, value, _)) = tuple((
        alt((
            tag("byr"), tag("iyr"), tag("eyr"), tag("hgt"), tag("hgt"), tag("hcl"), tag("ecl"), tag("pid"), tag("cid"),
        )),
        tag(":"),
        is_not(" \t\r\n"),
        opt(is_a(" \t\r\n"))
    ))(input)?;  
    Ok((input, (key, value)))
}
#[derive(Debug, Clone, Copy)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hcl: Option<RgbColor>,
    ecl: Option<EyeColor>,
    pid: Option<usize>,
}

fn passport(input: &str) -> IResult<&str, Passport> {
    let mut fields = HashMap::<&str, &str>::new();
    let mut new_input = input;
    while new_input != "" {
        let (input, (key, val)) = passport_tag(new_input)?;
        fields.insert(key, val);
        new_input = input;
    }
    
    let passport = Passport {
        byr: match fields.get("byr").map(|&x| x.parse::<usize>()) {
            Some(Ok(v)) if (v >= 1920 && v <= 2002) => Some(v),
            _ => None
        },
        iyr: match fields.get("iyr").map(|&x| x.parse::<usize>()) {
            Some(Ok(v)) if (v >= 2010 && v <= 2020) => Some(v),
            _ => None
        },
        eyr: match fields.get("eyr").map(|&x| x.parse::<usize>()) {
            Some(Ok(v)) if (v >= 2020 && v <= 2030) => Some(v),
            _ => None
        },
        ecl: match fields.get("ecl").map(|&x| x.parse::<EyeColor>()) {
            Some(Ok(v)) => Some(v),
            _ => None
        },
        hcl: match fields.get("hcl").map(|&x| rgb_color(x)) {
            Some(Ok((_, color))) => Some(color),
            _ => None
        },
        hgt: match fields.get("hgt").map(|&x| height(x)) {
            Some(Ok((_, Height {typ: HeightType::Inches, height}))) if height >= 59 && height <= 76 =>
                Some(Height {typ: HeightType::Inches, height}),
            Some(Ok((_, Height {typ: HeightType::Centimeters, height}))) if height >= 150 && height <= 193 => 
                Some(Height {typ: HeightType::Centimeters, height}),
            _ => None
        },
        pid: match fields.get("pid").map(|&x| if x.chars().count() == 9 {x.parse::<usize>().ok()} else {None}) {
            Some(Some(v)) if v < 1_000_000_000 => Some(v),
            _ => None
        },
    };
    
    Ok((new_input, passport))
}

pub fn stage4_1() -> usize {
    let mut f = std::fs::File::open("stage4.txt").unwrap();
    let mut passports = String::new();

    f.read_to_string(&mut passports).unwrap();
    passports.split("\n\n")
    .map(|p| passport(p))
    .filter_map(Result::ok)
    .filter(|(_, p)| p.byr.is_some() && p.iyr.is_some() && p.eyr.is_some() && 
                     p.hgt.is_some() && p.hcl.is_some() && p.ecl.is_some() && p.pid.is_some())
    .count()
}
