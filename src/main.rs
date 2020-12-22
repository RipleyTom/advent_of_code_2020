mod aoc_22;

fn main() {
    let input = aoc_22::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_22::run_a(&input));
    println!("Result of part B: {:?}", aoc_22::run_b(&input));
}
