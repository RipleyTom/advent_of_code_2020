mod aoc_08;

fn main() {
    let mut input = aoc_08::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_08::run_a(&mut input));
    println!("Result of part B: {:?}", aoc_08::run_b(&mut input));
}
