// 2020 Day 3

fn solve(input: &[u8], dx: usize, dy: usize) -> Result<usize, String> {
    input
        .split(|&x| x == b'\n')
        .take_while(|x| x.len() > 0)
        .enumerate()
        .try_fold(0usize, |acc, (i, elem)| {
            if i % dy != 0 {
                Ok(acc)
            } else {
                let y = i / dy;
                let x = y * dx % elem.len();
                Ok(acc + (elem[x] == '#' as u8) as usize)
            }
        })
}

pub fn part1(input: &[u8]) -> Result<String, String> {
    solve(input, 3, 1).map(|x| x.to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .try_fold(1usize, |acc, (x, y)| solve(input, *x, *y).map(|z| z * acc))
        .map(|x| x.to_string())
}
