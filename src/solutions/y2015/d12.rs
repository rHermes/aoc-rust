// 2015 Day 12

use serde_json;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let js: serde_json::Value = serde_json::from_slice(input).map_err(|e| e.to_string())?;

    let res = value(&js, false)?;

    Ok(res.to_string())
}

pub fn part2(input: &[u8]) -> core::result::Result<String, String> {
    let js: serde_json::Value = serde_json::from_slice(input).map_err(|e| e.to_string())?;

    let res = value(&js, true)?;

    Ok(res.to_string())
}

fn value(input: &serde_json::Value, part2: bool) -> Result<i64, String> {
    match input {
        serde_json::Value::Number(n) => n
            .as_i64()
            .ok_or("Couldn't format number from float".to_string()),
        serde_json::Value::Array(arr) => arr.iter().fold(Ok(0), |acc, c| {
            acc.and_then(|x| value(c, part2).map(|y| y + x))
        }),
        serde_json::Value::Object(m) => {
            if part2 && m.values().any(|x| x == "red") {
                return Ok(0);
            }
            m.values().fold(Ok(0), |acc, c| {
                acc.and_then(|x| value(c, part2).map(|y| y + x))
            })
        }
        _ => Ok(0),
    }
}
