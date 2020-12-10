mod aoc_10;

fn main() {
    let input = aoc_10::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_10::run_a(&input));
    println!("Result of part B: {:?}", aoc_10::run_b(&input));
}
