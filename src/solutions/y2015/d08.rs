// 2015 Day 8

use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut ans = 0;

    for line in sin.lines() {
        let mut mem = 0;
        let mut ll = line.chars();

        while let Some(v) = ll.next() {
            mem += 1;

            if v != '\\' {
                continue;
            }

            let nxt = ll.next().ok_or("Invalid input".to_string())?;

            if nxt == 'x' {
                ll.next().ok_or("Invalid input".to_string())?;
                ll.next().ok_or("Invalid input".to_string())?;
            } else if !(nxt == '\\' || nxt == '"') {
                return Err("Invalid input".to_string());
            }
        }

        ans += line.len() - (mem - 2);
    }
    Ok(ans.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let ans = sin.lines().fold(0, |acc, line| {
        acc + (2 + line.escape_default().count() - line.len())
    });
    Ok(ans.to_string())
}
