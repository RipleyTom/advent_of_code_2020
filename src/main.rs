mod aoc_06;

fn main() {
    let input = aoc_06::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_06::run_a(&input));
    println!("Result of part B: {:?}", aoc_06::run_b(&input));
}
