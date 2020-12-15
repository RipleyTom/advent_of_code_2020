mod aoc_15;

fn main() {
    let input = aoc_15::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_15::run_a(&input));
    println!("Result of part B: {:?}", aoc_15::run_b(&input));
}
