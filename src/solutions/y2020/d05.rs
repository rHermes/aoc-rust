use std::cmp::{max, min};

fn to_id(x: &[u8]) -> u16 {
    x.iter().rev().enumerate().fold(0, |a, (i, &x)| {
        if x == b'B' || x == b'R' {
            a + (1 << i)
        } else {
            a
        }
    })
}

pub fn part1(input: &[u8]) -> Result<String, String> {
    let ans = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .map(to_id)
        .max();

    if let Some(v) = ans {
        Ok(v.to_string())
    } else {
        Err("Input invalid!".to_string())
    }
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let (mi, ma, xo) = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .map(to_id)
        .fold((u16::MAX, u16::MIN, 0), |(mi, ma, xo), c| {
            (min(mi, c), max(ma, c), xo ^ c)
        });

    let ans = (mi..ma + 1).into_iter().fold(xo, |a, c| a ^ c);
    Ok(ans.to_string())
}
