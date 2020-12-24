mod aoc_24;

fn main() {
    let input = aoc_24::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_24::run_a(&input));
    println!("Result of part B: {:?}", aoc_24::run_b(&input));
}
