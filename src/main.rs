mod aoc_17;

fn main() {
    let input = aoc_17::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_17::run_a(&input));
    println!("Result of part B: {:?}", aoc_17::run_b(&input));
}
