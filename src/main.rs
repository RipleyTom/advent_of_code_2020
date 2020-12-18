mod aoc_18;

fn main() {
    let input = aoc_18::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_18::run_a(&input));
    println!("Result of part B: {:?}", aoc_18::run_b(&input));
}
