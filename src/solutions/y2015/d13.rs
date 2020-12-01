// 2015 Day 13

use std::collections::HashMap;
use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut g = HashMap::new();

    for line in sin.lines() {
        let lline = &line[..line.len() - 1];
        let mut it = lline.split_whitespace();
        let name = it.next().ok_or("Invalid input format!")?;
        let way = it.nth(1).ok_or("invalid input format!")?;
        let un = it
            .next()
            .ok_or("invalid inpuit format!")?
            .parse::<i64>()
            .map_err(|e| e.to_string())?;

        let other = it.last().ok_or("Invalid input format!")?;
        let n = if way != "gain" { -un } else { un };

        g.entry(name)
            .or_insert_with(|| HashMap::new())
            .insert(other, n);
    }

    let mut nms = g.keys().cloned().collect::<Vec<_>>();
    let eval = |conf: &Vec<&str>| -> i64 {
        let parscore: i64 = conf
            .windows(2)
            .map(|c| {
                let a = c.first().unwrap();
                let b = c.last().unwrap();

                g[*a][*b] + g[*b][*a]
            })
            .sum();
        let ff = conf.first().unwrap();
        let bb = conf.last().unwrap();

        parscore + (g[*ff][*bb] + g[*bb][*ff])
    };

    // Now we need to iterate through all lexographical orderings
    let mut c = vec![0usize; nms.len()];
    let mut max_pos = eval(&nms);

    let mut i = 0;
    while i < nms.len() {
        if c[i] < i {
            if (i % 2) == 0 {
                nms.swap(0, i);
            } else {
                nms.swap(c[i], i);
            }

            max_pos = max_pos.max(eval(&nms));

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    Ok(max_pos.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut g = HashMap::new();

    let me = "XXXXXXXXX";

    for line in sin.lines() {
        let lline = &line[..line.len() - 1];
        let mut it = lline.split_whitespace();
        let name = it.next().ok_or("Invalid input format!")?;
        let way = it.nth(1).ok_or("invalid input format!")?;
        let un = it
            .next()
            .ok_or("invalid inpuit format!")?
            .parse::<i64>()
            .map_err(|e| e.to_string())?;

        let other = it.last().ok_or("Invalid input format!")?;
        let n = if way != "gain" { -un } else { un };

        g.entry(name)
            .or_insert_with(|| HashMap::new())
            .insert(other, n);
        g.entry(name)
            .or_insert_with(|| HashMap::new())
            .insert(me, 0);
        g.entry(me)
            .or_insert_with(|| HashMap::new())
            .insert(name, 0);
    }

    let mut nms = g.keys().cloned().collect::<Vec<_>>();
    let eval = |conf: &Vec<&str>| -> i64 {
        let parscore: i64 = conf
            .windows(2)
            .map(|c| {
                let a = c.first().unwrap();
                let b = c.last().unwrap();

                g[*a][*b] + g[*b][*a]
            })
            .sum();
        let ff = conf.first().unwrap();
        let bb = conf.last().unwrap();

        parscore + (g[*ff][*bb] + g[*bb][*ff])
    };

    // Now we need to iterate through all lexographical orderings
    let mut c = vec![0usize; nms.len()];
    let mut max_pos = eval(&nms);

    let mut i = 0;
    while i < nms.len() {
        if c[i] < i {
            if (i % 2) == 0 {
                nms.swap(0, i);
            } else {
                nms.swap(c[i], i);
            }

            max_pos = max_pos.max(eval(&nms));

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    Ok(max_pos.to_string())
}
