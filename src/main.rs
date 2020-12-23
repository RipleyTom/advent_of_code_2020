mod aoc_23;

fn main() {
    let input = aoc_23::parse_input().unwrap();
    println!("Result of part A: {:?}", aoc_23::run_a(&input));
    println!("Result of part A(2): {:?}", aoc_23::run_a_impl_2(&input));
    println!("Result of part B: {:?}", aoc_23::run_b(&input));
}
