fn main() {
    let s = String::from("(()()");
    println!("What {}", aoc::solutions::y2015::d01::part1(s.as_bytes()));
}
