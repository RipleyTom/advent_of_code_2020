use std::collections::HashMap;

pub enum Entry {
	Mask(u64, u64, u64),
	Mem(u64, u64),
}

fn parse_data(input_data: &str) -> Result<Vec<u64>, std::io::Error> {
	Ok(input_data.split(',').map(|v| v.parse().unwrap()).collect())
}

pub fn parse_input() -> Result<Vec<u64>, std::io::Error> {
	parse_data("0,6,1,7,2,19,20")
}

pub fn run_a(input: &Vec<u64>) -> Result<u64, std::io::Error> {
	let mut map_last: HashMap<u64, (u64, u64)> = HashMap::new();
	let init_len = input.len() as u64;

	for i in 0..init_len {
		map_last.insert(input[i as usize], (i, i));
	}

	let mut last = input[(init_len - 1) as usize];

	for i in init_len..2020 {
		let (a, b) = map_last.get(&last).unwrap();
		last = b-a;

		let entry = map_last.entry(last).or_insert((i, i));
		entry.0 = entry.1;
		entry.1 = i;
	}

	Ok(last)
}

pub fn run_b(input: &Vec<u64>) -> Result<u64, std::io::Error> {
	let mut map_last: HashMap<u64, (u64, u64)> = HashMap::new();
	let init_len = input.len() as u64;

	for i in 0..init_len {
		map_last.insert(input[i as usize], (i, i));
	}

	let mut last = input[(init_len - 1) as usize];

	for i in init_len..30000000 {
		let (a, b) = map_last.get(&last).unwrap();
		last = b-a;

		let entry = map_last.entry(last).or_insert((i, i));
		entry.0 = entry.1;
		entry.1 = i;
	}

	Ok(last)
}

#[test]
fn test_aoc15() {
	let sample_input = "0,3,6";
	let sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(436));
	assert_eq!(run_b(&sample_res).ok(), Some(175594));
}
