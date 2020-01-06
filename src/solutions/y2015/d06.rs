// 2015 Day 6

use std::str;
pub fn part1(input: &[u8]) -> Result<String, String> {
    const N: usize = 1000;

    let mut grid = [[false; N]; N];

    // Parse the number cluster
    let parse_nums = |x: &[u8]| -> Result<(usize, usize), String> {
        let parts = x.split(|x| *x == b',');
        let first = parts.clone().nth(0).ok_or("Invalid input".to_string())?;
        let last = parts.clone().nth(1).ok_or("Invalid input".to_string())?;

        let nfirst = str::from_utf8(first)
            .map_err(|e| e.to_string())?
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let nlast = str::from_utf8(last)
            .map_err(|e| e.to_string())?
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;

        Ok((nfirst, nlast))
    };

    for line in input.split(|x| *x == b'\n') {
        if line == b"" {
            continue;
        }
        let parts = line.split(|x| *x == b' ');

        if line.starts_with(b"turn on ") {
            let first = parts.clone().nth(2).ok_or("Invalid input".to_string())?;
            let last = parts.clone().nth(4).ok_or("Invalid input".to_string())?;

            let (ax, ay) = parse_nums(first)?;
            let (bx, by) = parse_nums(last)?;

            for j in ay..(by + 1) {
                if let Some(row) = grid.get_mut(j) {
                    for i in ax..(bx + 1) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem = true;
                        } else {
                            return Err("Invalid input".to_string());
                        }
                    }
                } else {
                    return Err("Invalid input".to_string());
                }
            }
        } else if line.starts_with(b"turn off ") {
            let first = parts.clone().nth(2).ok_or("Invalid input".to_string())?;
            let last = parts.clone().nth(4).ok_or("Invalid input".to_string())?;

            let (ax, ay) = parse_nums(first)?;
            let (bx, by) = parse_nums(last)?;

            for j in ay..(by + 1) {
                if let Some(row) = grid.get_mut(j) {
                    for i in ax..(bx + 1) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem = false;
                        } else {
                            return Err("Invalid input".to_string());
                        }
                    }
                } else {
                    return Err("Invalid input".to_string());
                }
            }
        } else if line.starts_with(b"toggle ") {
            let first = parts.clone().nth(1).ok_or("Invalid input".to_string())?;
            let last = parts.clone().nth(3).ok_or("Invalid input".to_string())?;

            let (ax, ay) = parse_nums(first)?;
            let (bx, by) = parse_nums(last)?;

            for j in ay..(by + 1) {
                if let Some(row) = grid.get_mut(j) {
                    for i in ax..(bx + 1) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem = !*elem;
                        } else {
                            return Err("Invalid input".to_string());
                        }
                    }
                } else {
                    return Err("Invalid input".to_string());
                }
            }
        } else {
            return Err("Invalid input".to_string());
        }
    }

    let mut ans = 0;
    for j in 0..N {
        for i in 0..N {
            if grid[j][i] {
                ans += 1;
            }
        }
    }
    Ok(ans.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    const N: usize = 1000;

    let mut grid = [[0u32; N]; N];

    // Parse the number cluster
    let parse_nums = |x: &[u8]| -> Result<(usize, usize), String> {
        let parts = x.split(|x| *x == b',');
        let first = parts.clone().nth(0).ok_or("Invalid input".to_string())?;
        let last = parts.clone().nth(1).ok_or("Invalid input".to_string())?;

        let nfirst = str::from_utf8(first)
            .map_err(|e| e.to_string())?
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let nlast = str::from_utf8(last)
            .map_err(|e| e.to_string())?
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;

        Ok((nfirst, nlast))
    };

    for line in input.split(|x| *x == b'\n') {
        if line == b"" {
            continue;
        }
        let parts = line.split(|x| *x == b' ');

        if line.starts_with(b"turn on ") {
            let first = parts.clone().nth(2).ok_or("Invalid input".to_string())?;
            let last = parts.clone().nth(4).ok_or("Invalid input".to_string())?;

            let (ax, ay) = parse_nums(first)?;
            let (bx, by) = parse_nums(last)?;

            for j in ay..(by + 1) {
                if let Some(row) = grid.get_mut(j) {
                    for i in ax..(bx + 1) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem += 1;
                        } else {
                            return Err("Invalid input".to_string());
                        }
                    }
                } else {
                    return Err("Invalid input".to_string());
                }
            }
        } else if line.starts_with(b"turn off ") {
            let first = parts.clone().nth(2).ok_or("Invalid input".to_string())?;
            let last = parts.clone().nth(4).ok_or("Invalid input".to_string())?;

            let (ax, ay) = parse_nums(first)?;
            let (bx, by) = parse_nums(last)?;

            for j in ay..(by + 1) {
                if let Some(row) = grid.get_mut(j) {
                    for i in ax..(bx + 1) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem = elem.saturating_sub(1);
                        } else {
                            return Err("Invalid input".to_string());
                        }
                    }
                } else {
                    return Err("Invalid input".to_string());
                }
            }
        } else if line.starts_with(b"toggle ") {
            let first = parts.clone().nth(1).ok_or("Invalid input".to_string())?;
            let last = parts.clone().nth(3).ok_or("Invalid input".to_string())?;

            let (ax, ay) = parse_nums(first)?;
            let (bx, by) = parse_nums(last)?;

            for j in ay..(by + 1) {
                if let Some(row) = grid.get_mut(j) {
                    for i in ax..(bx + 1) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem += 2;
                        } else {
                            return Err("Invalid input".to_string());
                        }
                    }
                } else {
                    return Err("Invalid input".to_string());
                }
            }
        } else {
            return Err("Invalid input".to_string());
        }
    }

    let mut ans = 0usize;
    for j in 0..N {
        for i in 0..N {
            ans += grid[j][i] as usize;
        }
    }
    Ok(ans.to_string())
}
