pub mod y2015;
pub mod y2019;
pub mod y2020;

pub fn run_task(year: i32, day: i32, part: i32, input: &[u8]) -> Result<String, String> {
    match (year, day, part) {
        // 2015
        (2015, 1, 1) => y2015::d01::part1(input),
        (2015, 1, 2) => y2015::d01::part2(input),
        (2015, 2, 1) => y2015::d02::part1(input),
        (2015, 2, 2) => y2015::d02::part2(input),
        (2015, 3, 1) => y2015::d03::part1(input),
        (2015, 3, 2) => y2015::d03::part2(input),
        (2015, 4, 1) => y2015::d04::part1(input),
        (2015, 4, 2) => y2015::d04::part2(input),
        (2015, 5, 1) => y2015::d05::part1(input),
        (2015, 5, 2) => y2015::d05::part2(input),
        (2015, 6, 1) => y2015::d06::part1(input),
        (2015, 6, 2) => y2015::d06::part2(input),
        (2015, 7, 1) => y2015::d07::part1(input),
        (2015, 7, 2) => y2015::d07::part2(input),
        (2015, 8, 1) => y2015::d08::part1(input),
        (2015, 8, 2) => y2015::d08::part2(input),
        (2015, 9, 1) => y2015::d09::part1(input),
        (2015, 9, 2) => y2015::d09::part2(input),
        (2015, 10, 1) => y2015::d10::part1(input),
        (2015, 10, 2) => y2015::d10::part2(input),
        (2015, 11, 1) => y2015::d11::part1(input),
        (2015, 11, 2) => y2015::d11::part2(input),
        (2015, 12, 1) => y2015::d12::part1(input),
        (2015, 12, 2) => y2015::d12::part2(input),
        (2015, 13, 1) => y2015::d13::part1(input),
        (2015, 13, 2) => y2015::d13::part2(input),
        // 2020
        (2020, 1, 1) => y2020::d01::part1(input),
        (2020, 1, 2) => y2020::d01::part2(input),
        _ => Err("We don't have that task.".to_string()),
    }
}
