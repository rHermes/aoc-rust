// 2015 Day 7

use std::collections::HashMap;
use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut gs = HashMap::new();

    for l in sin.lines() {
        let mut lel = l.split(" -> ");

        let ins = lel.by_ref().next().ok_or("Invalid input".to_string())?;
        let outs = lel.by_ref().next().ok_or("Invalid input".to_string())?;

        gs.insert(outs, ins);
    }

    let mut cache = HashMap::new();
    eval(&gs, "a", &mut cache).map(|x| x.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let sin = str::from_utf8(input).map_err(|e| e.to_string())?;

    let mut gs = HashMap::new();

    for l in sin.lines() {
        let mut lel = l.split(" -> ");

        let ins = lel.by_ref().next().ok_or("Invalid input".to_string())?;
        let outs = lel.by_ref().next().ok_or("Invalid input".to_string())?;

        gs.insert(outs, ins);
    }

    let mut cache = HashMap::new();
    let tmp = eval(&gs, "a", &mut cache)?;

    cache.clear();
    cache.insert("b", tmp);
    eval(&gs, "a", &mut cache).map(|x| x.to_string())
}

fn eval<'a>(
    gs: &HashMap<&str, &'a str>,
    wire: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> Result<u16, String> {
    if let Some(v) = cache.get(wire) {
        return Ok(*v);
    }

    // first we just check if the wire is a number
    if let Ok(v) = u16::from_str_radix(wire, 10) {
        return Ok(v);
    }

    let raw = gs.get(wire).ok_or(format!("wire {} not found", wire))?;

    // We split it into parts.
    let parts = raw.split(" ").collect::<Vec<_>>();

    let ans = if parts.len() == 1 {
        eval(gs, parts[0], cache)
    } else if parts.len() == 2 && parts[0] == "NOT" {
        eval(gs, parts[1], cache).map(|x| !x)
    } else if parts.len() == 3 && parts[1] == "AND" {
        let a = eval(gs, parts[0], cache)?;
        let b = eval(gs, parts[2], cache)?;
        Ok(a & b)
    } else if parts.len() == 3 && parts[1] == "OR" {
        let a = eval(gs, parts[0], cache)?;
        let b = eval(gs, parts[2], cache)?;

        Ok(a | b)
    } else if parts.len() == 3 && parts[1] == "LSHIFT" {
        let a = eval(gs, parts[0], cache)?;
        let b = eval(gs, parts[2], cache)?;

        Ok(a << b)
    } else if parts.len() == 3 && parts[1] == "RSHIFT" {
        let a = eval(gs, parts[0], cache)?;
        let b = eval(gs, parts[2], cache)?;

        Ok(a >> b)
    } else {
        Err(format!("cannot evaluate wire {}: {}", wire, raw))
    };

    if let Ok(v) = ans {
        cache.insert(wire, v);
    }

    ans
}
