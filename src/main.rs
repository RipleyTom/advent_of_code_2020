mod aoc_16;

fn main() {
    let input = aoc_16::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_16::run_a(&input));
    println!("Result of part B: {:?}", aoc_16::run_b(&input));
}
