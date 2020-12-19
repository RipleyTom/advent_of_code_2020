mod aoc_19;

fn main() {
    let input = aoc_19::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_19::run_a(&input));
    println!("Result of part B: {:?}", aoc_19::run_b(&input));
}
