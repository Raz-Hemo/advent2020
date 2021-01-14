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
impl FromStr for RgbColor {
    type Err = anyhow::Error;
    fn from_str(i: &str) -> anyhow::Result<Self> {
        Ok(RgbColor {
            r: u8::from_str_radix(&i[0..2], 16)?,
            g: u8::from_str_radix(&i[2..4], 16)?,
            b: u8::from_str_radix(&i[4..6], 16)?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum EyeColor {Amb,Blu,Brn,Gry,Grn,Hzl,Oth}
impl FromStr for EyeColor {
    type Err = anyhow::Error;
    fn from_str(i: &str) -> anyhow::Result<Self> {
        match i {
            "amb" => Ok(EyeColor::Amb),
            "blu" => Ok(EyeColor::Blu),
            "brn" => Ok(EyeColor::Brn),
            "gry" => Ok(EyeColor::Gry),
            "grn" => Ok(EyeColor::Grn),
            "hzl" => Ok(EyeColor::Hzl),
            "oth" => Ok(EyeColor::Oth),
            _ => anyhow::bail!("Invalid eye color {}", i),
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
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn promote(self) -> anyhow::Result<Stage1Passport<'a>> {
        if let (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) = 
            (self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid) {
            Ok(Stage1Passport {byr,iyr,eyr,hgt,hcl,ecl,pid})
        } else {
            anyhow::bail!("Can't create stage 1 passport: missing fields")
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Stage1Passport<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
}

impl<'a> Stage1Passport<'a> {
    fn promote(&self) -> anyhow::Result<Stage2Passport> {
        Ok(Stage2Passport{
            byr: match self.byr.parse::<usize>()? {
                byr if byr >= 1920 && byr <= 2002 => byr,
                _ => anyhow::bail!("Invalid byr {}", self.byr)
            },
            iyr: match self.iyr.parse::<usize>()? {
                iyr if iyr >= 2010 && iyr <= 2020 => iyr,
                _ => anyhow::bail!("Invalid iyr {}", self.iyr)
            },
            eyr: match self.eyr.parse::<usize>()? {
                eyr if eyr >= 2020 && eyr <= 2030 => eyr,
                _ => anyhow::bail!("Invalid eyr {}", self.eyr)
            },
            hgt: match height(self.hgt).map_err(|_| anyhow::anyhow!("invalid height {}", self.hgt))? {
                (_, Height {typ: HeightType::Inches, height}) if height >= 59 && height <= 76 =>
                    Height {typ: HeightType::Inches, height},
                (_, Height {typ: HeightType::Centimeters, height}) if height >= 150 && height <= 193 => 
                    Height {typ: HeightType::Centimeters, height},
                _ => anyhow::bail!("Invalid height {}", self.hgt)
            },
            hcl: rgb_color(self.hcl).map_err(|_| anyhow::anyhow!("invalid hcl {}", self.hcl))?.1,
            ecl: self.ecl.parse::<EyeColor>()?,
            pid: match self.pid.parse::<usize>()? {
                pid if self.pid.chars().count() == 9 => pid,
                _ => anyhow::bail!("invalid pid: {}", self.pid)
            },
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Stage2Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Height,
    hcl: RgbColor,
    ecl: EyeColor,
    pid: usize,
}

fn passport(input: &str) -> IResult<&str, Passport> {
    let mut fields = HashMap::<&str, &str>::new();
    let mut new_input = input;
    while new_input != "" {
        let (input, (key, val)) = passport_tag(new_input)?;
        fields.insert(key, val);
        new_input = input;
    }

    Ok((new_input, Passport {
        // Dereference inner value to make &&str->&str
        byr: fields.get("byr").map(|f| *f),
        iyr: fields.get("iyr").map(|f| *f),
        eyr: fields.get("eyr").map(|f| *f),
        ecl: fields.get("ecl").map(|f| *f),
        hcl: fields.get("hcl").map(|f| *f),
        hgt: fields.get("hgt").map(|f| *f),
        pid: fields.get("pid").map(|f| *f),
    }))
}

pub fn stage4_1() -> usize {
    let mut f = std::fs::File::open("stage4.txt").unwrap();
    let mut passports = String::new();

    f.read_to_string(&mut passports).unwrap();
    passports.split("\n\n")
    .map(|p| passport(p))
    .filter_map(Result::ok)
    .filter_map(|(_, p)| p.promote().ok())
    .count()
}


pub fn stage4_2() -> usize {
    let mut f = std::fs::File::open("stage4.txt").unwrap();
    let mut passports = String::new();

    f.read_to_string(&mut passports).unwrap();
    passports.split("\n\n")
    .map(|p| passport(p))
    .filter_map(Result::ok)
    .filter_map(|(_, p)| p.promote().ok())
    .filter_map(|p| p.promote().ok())
    .count()
}
