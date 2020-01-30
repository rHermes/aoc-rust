// 2015 Day 11

use std::char;
use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    solve(input, 1)
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    solve(input, 2)
}

fn solve(input: &[u8], mut times: i32) -> Result<String, String> {
    let mut sin = str::from_utf8(input)
        .map_err(|e| e.to_string())?
        .trim_end()
        .chars()
        .map(|x| x.to_digit(36).and_then(|y| y.checked_sub(9)))
        .collect::<Option<Vec<_>>>()
        .ok_or("Invalid input".to_string())?;

    while times > 0 {
        for k in sin.iter_mut().rev() {
            *k += 1;

            if *k > 26 {
                *k = 1;
            } else {
                break;
            }
        }
        if valid(&sin) {
            times -= 1;
        }
    }

    let ans = sin
        .iter()
        .map(|x| char::from_digit(x + 9, 36))
        .collect::<Option<String>>()
        .ok_or("Should not happen".to_string())?;
    Ok(ans)
}

fn valid(v: &Vec<u32>) -> bool {
    if v.iter().any(|x| [12, 9, 15].contains(x)) {
        return false;
    }
    if !v.as_slice().windows(3).any(|x| {
        let mut mit = x.iter();
        let a = mit.next().unwrap();
        let b = mit.next().unwrap();
        let c = mit.next().unwrap();

        (*c == (*b + 1)) && (*b == (*a + 1))
    }) {
        return false;
    }

    let mut it = v.as_slice().windows(2);

    while let Some(w) = it.next() {
        if w.first() != w.last() {
            continue;
        }
        let k = it.clone().skip(1).any(|z| z.first() == z.last());

        if k {
            return true;
        }
    }

    false
}
