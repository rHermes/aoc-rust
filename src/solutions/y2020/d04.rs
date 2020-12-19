// 2020 Day 4

use std::str;

// We need it for the .batching
use itertools::Itertools;

use bitflags::bitflags;

bitflags! {
    struct Field: u8 {
        const ERR = 1 << 0;
        const BYR = 1 << 1;
        const IYR = 1 << 2;
        const EYR = 1 << 3;
        const HGT = 1 << 4;
        const HCL = 1 << 5;
        const ECL = 1 << 6;
        const PID = 1 << 7;
    }
}

fn valid_num(v: &[u8], low: u16, high: u16, len: usize, pos: Field) -> Field {
    (v.len() == len)
        .then_some(v)
        .and_then(|x| str::from_utf8(x).ok())
        .and_then(|x| x.parse::<u16>().ok())
        .filter(|&x| low <= x && x <= high)
        .map_or(Field::empty(), |_| pos)
}

fn valid_hgt(v: &[u8]) -> Field {
    match &v[v.len() - 2..v.len()] {
        b"cm" => valid_num(&v[..v.len() - 2], 150, 193, 3, Field::HGT),
        b"in" => valid_num(&v[..v.len() - 2], 59, 76, 2, Field::HGT),
        _ => Field::ERR,
    }
}

fn valid_ecl(v: &[u8]) -> Field {
    match v {
        b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth" => Field::ECL,
        _ => Field::ERR,
    }
}

fn valid_pid(v: &[u8]) -> Field {
    if v.len() == 9 && v.iter().all(|x| x.is_ascii_digit()) {
        Field::PID
    } else {
        Field::ERR
    }
}

fn valid_hcl(v: &[u8]) -> Field {
    if v.len() == 7 && v[1..].iter().all(|x| x.is_ascii_hexdigit()) {
        Field::HCL
    } else {
        Field::ERR
    }
}

pub fn part1(input: &[u8]) -> Result<String, String> {
    let lines = input
        .split(|&x| x == b'\n' || x == b' ')
        .batching(|it| {
            it.take_while(|x| x.len() > 0).fold(None, |a, c| {
                Some(
                    a.unwrap_or(Field::empty())
                        | match c[..4].as_ref() {
                            b"byr:" => Field::BYR,
                            b"iyr:" => Field::IYR,
                            b"eyr:" => Field::EYR,
                            b"hgt:" => Field::HGT,
                            b"hcl:" => Field::HCL,
                            b"ecl:" => Field::ECL,
                            b"pid:" => Field::PID,
                            b"cid:" => Field::empty(),
                            _ => Field::ERR,
                        },
                )
            })
        })
        .filter(|&x| x == (Field::all() - Field::ERR))
        .count();
    Ok(lines.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let lines = input
        .split(|&x| x == b'\n' || x == b' ')
        .batching(|it| {
            it.take_while(|x| x.len() > 0)
                .map(|x| x.split_at(4))
                .fold(None, |a, (c, v)| {
                    Some(
                        a.unwrap_or(Field::empty())
                            | match c {
                                b"byr:" => valid_num(v, 1920, 2002, 4, Field::BYR),
                                b"iyr:" => valid_num(v, 2010, 2020, 4, Field::IYR),
                                b"eyr:" => valid_num(v, 2020, 2030, 4, Field::EYR),
                                b"hgt:" => valid_hgt(v),
                                b"hcl:" => valid_hcl(v),
                                b"ecl:" => valid_ecl(v),
                                b"pid:" => valid_pid(v),
                                b"cid:" => Field::empty(),
                                _ => Field::ERR,
                            },
                    )
                })
        })
        .filter(|&x| x == (Field::all() - Field::ERR))
        .count();

    Ok(lines.to_string())
}
