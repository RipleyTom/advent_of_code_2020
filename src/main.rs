mod aoc_20;

fn main() {
    let input = aoc_20::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_20::run_a(&input));
    println!("Result of part B: {:?}", aoc_20::run_b(&input));
}
