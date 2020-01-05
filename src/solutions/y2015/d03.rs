// 2015 Day 3
use std::collections::HashSet;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let mut seen = HashSet::new();

    let mut sx = 0;
    let mut sy = 0;

    seen.insert((sx, sy));

    for c in input.iter() {
        match c {
            b'^' => sy += 1,
            b'>' => sx += 1,
            b'v' => sy -= 1,
            b'<' => sx -= 1,
            _ => return Err("Invalid character in input".to_string()),
        };

        seen.insert((sx, sy));
    }

    Ok(seen.len().to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let mut seen = HashSet::new();

    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;

    seen.insert((sx, sy));

    for (c, &t) in input.iter().zip([true, false].iter().cycle()) {
        if t {
            match c {
                b'^' => sy += 1,
                b'>' => sx += 1,
                b'v' => sy -= 1,
                b'<' => sx -= 1,
                _ => return Err("Invalid character in input".to_string()),
            };
            seen.insert((sx, sy));
        } else {
            match c {
                b'^' => ry += 1,
                b'>' => rx += 1,
                b'v' => ry -= 1,
                b'<' => rx -= 1,
                _ => return Err("Invalid character in input".to_string()),
            };
            seen.insert((rx, ry));
        }
    }

    Ok(seen.len().to_string())
}
