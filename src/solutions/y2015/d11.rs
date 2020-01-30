// 2015 Day 11

use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input)
        .map_err(|e| e.to_string())?
        .trim_end()
        .chars()
        .map(|x| x.to_digit(36))
        .collect::<Option<Vec<_>>>()
        .ok_or("Invalid input".to_string())?;

    Err("Not implemented yet".to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    Err("Not implemented yet".to_string())
}
