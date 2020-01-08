// 2015 Day 9

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut g = HashMap::new();

    for line in sin.lines() {
        let mut parts = line.split(" = ");
        let mut towns = parts
            .next()
            .ok_or("Invalid input".to_string())?
            .split(" to ");
        let score = parts
            .next()
            .ok_or("Invalid input".to_string())?
            .parse::<u32>()
            .map_err(|x| x.to_string())?;

        let ta = towns.next().ok_or("Invalid input".to_string())?;
        let tb = towns.next().ok_or("Invalid input".to_string())?;

        // Insert into the hashmaps
        g.entry(ta)
            .or_insert_with(|| HashMap::new())
            .insert(tb, score);
        g.entry(tb)
            .or_insert_with(|| HashMap::new())
            .insert(ta, score);
    }

    let ss = HashSet::from_iter(g.keys().copied());

    let ans = ss
        .iter()
        .map(|&x| solve1(&g, x, &ss))
        .min()
        .ok_or("This is not supposed to happen".to_string())?;
    Ok(ans.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut g = HashMap::new();

    for line in sin.lines() {
        let mut parts = line.split(" = ");
        let mut towns = parts
            .next()
            .ok_or("Invalid input".to_string())?
            .split(" to ");
        let score = parts
            .next()
            .ok_or("Invalid input".to_string())?
            .parse::<u32>()
            .map_err(|x| x.to_string())?;

        let ta = towns.next().ok_or("Invalid input".to_string())?;
        let tb = towns.next().ok_or("Invalid input".to_string())?;

        // Insert into the hashmaps
        g.entry(ta)
            .or_insert_with(|| HashMap::new())
            .insert(tb, score);
        g.entry(tb)
            .or_insert_with(|| HashMap::new())
            .insert(ta, score);
    }

    let ss = HashSet::from_iter(g.keys().copied());

    let ans = ss
        .iter()
        .map(|&x| solve2(&g, x, &ss))
        .max()
        .ok_or("This is not supposed to happen".to_string())?;
    Ok(ans.to_string())
}

fn solve1(g: &HashMap<&str, HashMap<&str, u32>>, start: &str, left: &HashSet<&str>) -> u32 {
    let kk = HashSet::from_iter(left.iter().copied().filter(|y| **y != *start));
    kk.iter()
        .map(|&x| g[start][x] + solve1(g, x, &kk))
        .min()
        .unwrap_or(0)
}

fn solve2(g: &HashMap<&str, HashMap<&str, u32>>, start: &str, left: &HashSet<&str>) -> u32 {
    let kk = HashSet::from_iter(left.iter().copied().filter(|y| **y != *start));
    kk.iter()
        .map(|&x| g[start][x] + solve2(g, x, &kk))
        .max()
        .unwrap_or(0)
}
