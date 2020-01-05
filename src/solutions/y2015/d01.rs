// 2015 Day 1
pub fn part1(input: &[u8]) -> Result<String, String> {
    let mut score: i64 = 0;

    for c in input.iter() {
        match c {
            b'(' => score += 1,
            b')' => score -= 1,
            _ => return Err("Invalid character in input".to_string()),
        }
    }

    Ok(score.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let mut score: i64 = 0;

    for (i, c) in input.iter().enumerate() {
        match c {
            b'(' => score += 1,
            b')' => score -= 1,
            _ => return Err("Invalid character in input".to_string()),
        }
        if score == -1 {
            return Ok((i + 1).to_string());
        }
    }

    Err("We never entered the basement".to_string())
}
