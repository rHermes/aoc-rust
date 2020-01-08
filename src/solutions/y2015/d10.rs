// 2015 Day 10

use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input)
        .map_err(|e| e.to_string())?
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .ok_or("Invalid input".to_string())?;

    let mut prev = sin;
    for _ in 0..40 {
        prev = look_and_say(&prev);
    }
    Ok(prev.len().to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input)
        .map_err(|e| e.to_string())?
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .ok_or("Invalid input".to_string())?;

    let mut prev = sin;
    for _ in 0..50 {
        prev = look_and_say(&prev);
    }
    Ok(prev.len().to_string())
}

fn look_and_say(prev: &Vec<u32>) -> Vec<u32> {
    let mut nw = Vec::with_capacity((prev.len() * 130) / 100);
    let mut it = prev.iter().peekable();

    while let Some(&v) = it.next() {
        let mut n = 1;
        while let Some(&w) = it.peek() {
            if *w != v {
                break;
            }

            n += 1;
            it.next();
        }
        nw.push(n);
        nw.push(v);
    }

    nw
}
