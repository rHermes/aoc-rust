// 2020 Day 2

use std::str;

struct Line<'a> {
    low: usize,
    high: usize,
    c: u8,
    password: &'a [u8],
}

fn parse_input(input: &[u8]) -> Result<Line, String> {
    let mut prts = input.splitn(3, |x| *x == b' ');
    let b_range = prts.next().ok_or("Invalid format".to_string())?;
    let b_char = prts.next().ok_or("Invalid format".to_string())?;
    let b_pass = prts.next().ok_or("Invalid format".to_string())?;

    let mut rparts = b_range.splitn(2, |x| *x == b'-');
    let b_low = rparts.next().ok_or("Invalid format".to_string())?;
    let b_hi = rparts.next().ok_or("Invalid format".to_string())?;

    let s_lo = str::from_utf8(b_low).map_err(|_| "invalid format".to_string())?;
    let s_hi = str::from_utf8(b_hi).map_err(|_| "invalid format".to_string())?;

    let lo = s_lo.parse().map_err(|_| "invalid format".to_string())?;
    let hi = s_hi.parse().map_err(|_| "invalid format".to_string())?;

    Ok(Line {
        low: lo,
        high: hi,
        c: b_char[0],
        password: b_pass,
    })
}

pub fn part1(input: &[u8]) -> Result<String, String> {
    let ans = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .fold(Ok(0usize), |acc: Result<usize, String>, x| {
            if acc.is_err() {
                return acc;
            }

            let line = parse_input(x)?;

            let i = line.password.iter().filter(|x| **x == line.c).count();

            if line.low <= i && i <= line.high {
                acc.map(|x| x + 1)
            } else {
                acc
            }
        })?;

    Ok(ans.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let ans = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .fold(Ok(0usize), |acc: Result<usize, String>, x| {
            if acc.is_err() {
                return acc;
            }

            let line = parse_input(x)?;

            let a = line
                .password
                .get(line.low - 1)
                .map(|x| *x == line.c)
                .ok_or("invalid format".to_string())?;
            let b = line
                .password
                .get(line.high - 1)
                .map(|x| *x == line.c)
                .ok_or("invalid format".to_string())?;
            if (a && !b) || (!a && b) {
                acc.map(|x| x + 1)
            } else {
                acc
            }
        })?;

    Ok(ans.to_string())
}
