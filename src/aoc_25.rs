use std::fs::File;
use std::io::Read;

type PuzzleData = (u64, u64);

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	let parsed: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
	Ok((parsed[0], parsed[1]))
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_25.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

fn handshake(mut value: u64, subject_number: u64) -> u64 {
	value *= subject_number;
	value %= 20201227;
	value
}

fn get_num_loops_for_key(subject: u64, key: u64) -> u64 {
	let mut cur_value = subject;
	let mut num_loops = 1;

	loop {
		if cur_value == key {
			return num_loops;
		}
		cur_value = handshake(cur_value, subject);
		num_loops += 1;
	}
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {

	let num_loops_1 = get_num_loops_for_key(7, input.0);
	let num_loops_2 = get_num_loops_for_key(7, input.1);

	println!("Num loops: {} and {}", num_loops_1, num_loops_2);

	let mut enc_key = 1;
	for _ in 0..num_loops_2 {
		enc_key = handshake(enc_key, input.0);
	}

	Ok(enc_key)
}

#[test]
fn test_aoc25() {
	let sample_res = parse_data(&"5764801\n17807724").unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(14897079));
}
