mod aoc_14;

fn main() {
    let input = aoc_14::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_14::run_a(&input));
    println!("Result of part B: {:?}", aoc_14::run_b(&input));
}
