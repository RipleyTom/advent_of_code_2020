mod aoc_13;

fn main() {
    let input = aoc_13::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_13::run_a(&input));
    println!("Result of part B: {:?}", aoc_13::run_b(&input));
}
