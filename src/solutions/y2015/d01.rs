pub fn part1(input: &[u8]) -> String {
    let s  = String::from_utf8_lossy(input); // # .unwrap();
    let k = s.chars().fold(0, |score, c| {
                match c {
                    '(' => score + 1,
                    ')' => score - 1,
                    _ => score
                }
        });
        
    k.to_string()
}

pub fn part2(input: Vec<u8>) -> String {
    "World!".to_string()
}
