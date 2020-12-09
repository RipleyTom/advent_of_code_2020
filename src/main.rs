mod aoc_09;

fn main() {
    let input = aoc_09::parse_input().unwrap();
    let res_a = aoc_09::run_a(&input, 25);
    println!("Result of part A: {:?}", aoc_09::run_a(&input, 25));
    println!("Result of part B: {:?}", aoc_09::run_b(&input, res_a.unwrap()));
}
