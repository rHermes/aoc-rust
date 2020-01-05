// 2015 Day 4
use md5::{Digest, Md5};
use std::io::Write;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let mut hasher = Md5::new();

    let rinput = input
        .iter()
        .take_while(|&&x| x != b'\n')
        .copied()
        .collect::<Vec<u8>>();

    let mut buf = Vec::with_capacity(8);
    let ans = (0..u32::max_value())
        .filter_map(|x| {
            buf.clear();
            write!(buf, "{}", x).unwrap();

            hasher.input(&rinput);
            hasher.input(&buf);

            let dgst = hasher.result_reset();

            if dgst[..2] == [0, 0] && dgst[2] < 0x10 {
                Some(x.to_string())
            } else {
                None
            }
        })
        .next();

    ans.ok_or("No such hash found!".to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    let mut hasher = Md5::new();

    let rinput = input
        .iter()
        .take_while(|&&x| x != b'\n')
        .copied()
        .collect::<Vec<u8>>();

    let mut buf = Vec::with_capacity(8);
    let ans = (0..u32::max_value())
        .filter_map(|x| {
            buf.clear();
            write!(buf, "{}", x).unwrap();

            hasher.input(&rinput);
            hasher.input(&buf);

            let dgst = hasher.result_reset();

            if dgst[..3] == [0, 0, 0] {
                Some(x.to_string())
            } else {
                None
            }
        })
        .next();

    ans.ok_or("No such hash found!".to_string())
}
