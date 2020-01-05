pub mod y2015;
pub mod y2019;

pub fn run_task(year: i32, day: i32, part: i32, input: &[u8]) -> Result<String, String> {
    match (year, day, part) {
        (2015, 1, 1) => y2015::d01::part1(input),
        (2015, 1, 2) => y2015::d01::part2(input),
        (2015, 2, 1) => y2015::d02::part1(input),
        (2015, 2, 2) => y2015::d02::part2(input),
        (2015, 3, 1) => y2015::d03::part1(input),
        (2015, 3, 2) => y2015::d03::part2(input),
        _ => Err("We don't have that task.".to_string()),
    }
}
