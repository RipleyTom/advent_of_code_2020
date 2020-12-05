use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
struct BoardingPass {
	row: i64,
	column: i64,
	id: i64,
}

fn convert_bp(input: &str) -> Result<BoardingPass, ()> {
	if input.len() != 10 {
		return Err(());
	}

	let mut row = 0;
	for i in 0..7 {
		row *= 2;
		match input.chars().nth(i).unwrap() {
			'B' => row += 1,
			'F' => {}
			_ => return Err(()),
		}
	}

	let mut column = 0;
	for i in 7..10 {
		column *= 2;
		match input.chars().nth(i).unwrap() {
			'R' => column += 1,
			'L' => {}
			_ => return Err(()),
		}
	}

	Ok(BoardingPass {
		row,
		column,
		id: (row * 8 + column),
	})
}

fn parse_input() -> Result<Vec<BoardingPass>, std::io::Error> {
	let file = File::open("input_05.txt")?;
	BufReader::new(file)
		.lines()
		.map(|l| convert_bp(&l.unwrap()))
		.collect::<Result<Vec<BoardingPass>, _>>()
		.map_err(|_| {
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Invalid character on the map!",
			)
		})
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	Ok(input.iter().map(|bp| bp.id).max().unwrap())
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let mut input_id = input.iter().map(|bp| bp.id).collect::<Vec<i64>>();
	input_id.sort();

	for i in 1..input_id.len() {
		if input_id[i - 1] == (input_id[i] - 2) {
			return Ok(input_id[i] - 1);
		}
	}

	Err(std::io::Error::new(
		std::io::ErrorKind::InvalidData,
		"No boarding pass ID gap found!",
	))
}

#[test]
fn test_aoc05() {
	assert_eq!(convert_bp("BFFFBBFRRR").unwrap().id, 567);
	assert_eq!(convert_bp("FFFBBBFRRR").unwrap().id, 119);
	assert_eq!(convert_bp("BBFFBBFRLL").unwrap().id, 820);
}
