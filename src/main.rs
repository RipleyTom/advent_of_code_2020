mod aoc_12;

fn main() {
    let input = aoc_12::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_12::run_a(&input));
    println!("Result of part B: {:?}", aoc_12::run_b(&input));
}
