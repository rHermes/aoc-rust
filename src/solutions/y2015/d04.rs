// 2015 Day 4
use md5;

pub fn part1(input: &[u8]) -> Result<String, String> {
    let mut base_ctx = md5::Context::new();

    base_ctx.consume(
        input
            .iter()
            .take_while(|&&x| x != b'\n')
            .copied()
            .collect::<Vec<u8>>(),
    );

    let ans = (0..u32::max_value())
        .filter_map(|x| {
            let mut nctx = base_ctx.clone();
            nctx.consume(x.to_string().as_bytes());
            let dgst = nctx.compute();

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
    let mut base_ctx = md5::Context::new();

    // This is to remove possible new lines
    base_ctx.consume(
        input
            .iter()
            .take_while(|&&x| x != b'\n')
            .copied()
            .collect::<Vec<u8>>(),
    );

    let ans = (0..u32::max_value())
        .filter_map(|x| {
            let mut nctx = base_ctx.clone();
            nctx.consume(x.to_string().as_bytes());
            let dgst = nctx.compute();

            if dgst[..3] == [0, 0, 0] {
                Some(x.to_string())
            } else {
                None
            }
        })
        .next();

    ans.ok_or("No such hash found!".to_string())
}
