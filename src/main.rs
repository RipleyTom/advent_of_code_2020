mod aoc_25;

fn main() {
    let input = aoc_25::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_25::run_a(&input));
}
