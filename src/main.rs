mod aoc_07;

fn main() {
    let input = aoc_07::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_07::run_a(&input));
    println!("Result of part B: {:?}", aoc_07::run_b(&input));
}
