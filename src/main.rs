mod aoc_21;

fn main() {
    let input = aoc_21::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_21::run_a(&input));
    println!("Result of part B: {:?}", aoc_21::run_b(&input));
}
