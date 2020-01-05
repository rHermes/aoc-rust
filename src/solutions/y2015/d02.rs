// 2015 Day 2
pub fn part1(input: &[u8]) -> Result<String, String> {
    let mut score: u32 = 0;

    let mut iter = input.iter().peekable();

    while let Some(_) = iter.peek() {
        let mut king = iter.by_ref().take_while(|&&x| x != b'\n');

        // Convert
        let l = king
            .by_ref()
            .take_while(|&&x| x != b'x')
            .fold(0, |acc, &x| acc * 10 + ((x - b'0') as u32));
        let w = king
            .by_ref()
            .take_while(|&&x| x != b'x')
            .fold(0, |acc, &x| acc * 10 + ((x - b'0') as u32));
        let h = king
            .by_ref()
            .take_while(|&&x| x != b'x')
            .fold(0, |acc, &x| acc * 10 + ((x - b'0') as u32));

        // Calculate sides
        let lw = l * w;
        let lh = l * h;
        let wh = w * h;

        score += 2 * lw + 2 * lh + 2 * wh + lw.min(lh.min(wh));
    }

    Ok(score.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let mut score: u32 = 0;

    let mut iter = input.iter().peekable();

    while let Some(_) = iter.peek() {
        let mut king = iter.by_ref().take_while(|&&x| x != b'\n');

        // Convert
        let l = king
            .by_ref()
            .take_while(|&&x| x != b'x')
            .fold(0, |acc, &x| acc * 10 + ((x - b'0') as u32));
        let w = king
            .by_ref()
            .take_while(|&&x| x != b'x')
            .fold(0, |acc, &x| acc * 10 + ((x - b'0') as u32));
        let h = king
            .by_ref()
            .take_while(|&&x| x != b'x')
            .fold(0, |acc, &x| acc * 10 + ((x - b'0') as u32));

        // Calculate sides
        let m = l.max(w.max(h));

        let k = if m == l {
            2 * w + 2 * h
        } else if m == w {
            2 * l + 2 * h
        } else {
            2 * w + 2 * l
        };

        score += l * w * h + k;
    }

    Ok(score.to_string())
}
