mod aoc_11;

fn main() {
    let input = aoc_11::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_11::run_a(&input));
    println!("Result of part B: {:?}", aoc_11::run_b(&input));
}
