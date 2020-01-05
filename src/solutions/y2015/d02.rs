// 2015 Day 2
pub fn part1(input: &[u8]) -> Result<String, String> {
    let ans = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .fold(Ok(0), |acc, sub| {
            if acc.is_err() {
                return acc;
            }
            let mut numbers = sub
                .split(|&x| x == b'x')
                .map(|x| x.iter().fold(0, |nacc, &y| nacc * 10 + ((y - b'0') as u32)));

            // No safe way to unpack this so we have to drop spead
            let l = numbers.next().ok_or("Invalid input".to_string())?;
            let w = numbers.next().ok_or("Invalid input".to_string())?;
            let h = numbers.next().ok_or("Invalid input".to_string())?;

            let lw = l * w;
            let lh = l * h;
            let wh = w * h;

            acc.map(|acc| acc + 2 * lw + 2 * lh + 2 * wh + lw.min(lh.min(wh)))
        });

    ans.map(|x| x.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let ans = input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .fold(Ok(0), |acc, sub| {
            if acc.is_err() {
                return acc;
            }
            let mut numbers = sub
                .split(|&x| x == b'x')
                .map(|x| x.iter().fold(0, |nacc, &y| nacc * 10 + ((y - b'0') as u32)));

            // No safe way to unpack this so we have to drop spead
            let l = numbers.next().ok_or("Invalid input".to_string())?;
            let w = numbers.next().ok_or("Invalid input".to_string())?;
            let h = numbers.next().ok_or("Invalid input".to_string())?;

            // Calculate sides
            let m = l.max(w.max(h));

            let k = if m == l {
                2 * w + 2 * h
            } else if m == w {
                2 * l + 2 * h
            } else {
                2 * w + 2 * l
            };

            acc.map(|acc| acc + l * w * h + k)
        });

    ans.map(|x| x.to_string())
}
