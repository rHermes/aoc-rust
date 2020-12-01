// 2020 Day 1

// use std::str;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let nums = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .map(|x| x.iter().fold(0, |nacc, &y| nacc * 10 + ((y - b'0') as u32)))
        .collect::<Vec<_>>();

    for x in 0..nums.len() {
        for y in x..nums.len() {
            if nums[x] + nums[y] == 2020 {
                return Ok((nums[x] * nums[y]).to_string());
            }
        }
    }

    Err("No valid solutions".to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let nums = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .map(|x| x.iter().fold(0, |nacc, &y| nacc * 10 + ((y - b'0') as u32)))
        .collect::<Vec<_>>();

    for x in 0..nums.len() {
        for y in x..nums.len() {
            if nums[x] + nums[y] >= 2020 {
                continue
            }
            for z in y..nums.len() {
                if nums[x] + nums[y] + nums[z] == 2020 {
                    return Ok((nums[x] * nums[y] * nums[z]).to_string());
                }
            }
        }
    }

    Err("No valid solutions".to_string())
}
