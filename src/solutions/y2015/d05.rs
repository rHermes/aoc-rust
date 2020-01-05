// 2015 Day 5
pub fn part1(input: &[u8]) -> Result<String, String> {
    let lines = input
        .split(|x| *x == b'\n')
        .filter(|x| {
            !x.windows(2).any(|x| match x {
                b"ab" | b"cd" | b"pq" | b"xy" => true,
                _ => false,
            })
        })
        .filter(|x| x.iter().zip(x.iter().skip(1)).any(|(a, b)| *a == *b))
        .filter(|x| {
            x.iter()
                .filter(|x| match x {
                    b'a' | b'e' | b'i' | b'o' | b'u' => true,
                    _ => false,
                })
                .count()
                >= 3
        });

    Ok(lines.count().to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let lines = input
        .split(|x| *x == b'\n')
        .filter(|x| x.windows(3).any(|x| x.first() == x.last()))
        .filter(|x| {
            let mut it = x.windows(2);

            while let Some(v) = it.next() {
                let k = it.clone().skip(1).any(|z| v.eq(z));

                if k {
                    return true;
                }
            }

            false
        });

    Ok(lines.count().to_string())
}
