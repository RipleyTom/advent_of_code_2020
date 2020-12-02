use std::fs::File;
use std::io::{BufRead, BufReader};

struct InputData {
	min: i64,
	max: i64,
	character: char,
	password: String,
}

fn parse_input() -> Result<Vec<InputData>, std::io::Error> {
	let file = File::open("input_02.txt")?;

	let result: Result<Vec<InputData>, _> = BufReader::new(file)
		.lines()
		.map(|line| {
			let line = line.unwrap();
			let values: Vec<&str> = line.trim().split(' ').collect();
			if values.len() != 3 || values[1].len() != 2 {
				return Err(());
			}
			let min_max: Vec<i64> = values[0]
				.split('-')
				.map(|mm| mm.parse::<i64>().or_else(|_| Err(())))
				.collect::<Result<Vec<i64>, _>>()?;
			if min_max.len() != 2 {
				return Err(());
			}
			let dachar = values[1].chars().nth(0).unwrap();
			Ok(InputData {
				min: min_max[0],
				max: min_max[1],
				character: dachar,
				password: String::from(values[2]),
			})
		})
		.collect();
	result.map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid input!"))
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;
	let mut valid_input = 0;

	for i in input {
		let num_chars: i64 = i
			.password
			.chars()
			.map(|c| c == i.character)
			.filter(|b| *b)
			.count() as i64;
		if num_chars >= i.min && num_chars <= i.max {
			valid_input += 1;
		}
	}

	Ok(valid_input)
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;
	let mut valid_input = 0;

	for i in input {
		let mut count = 0;

		let mut check_pos = |p: i64| {
			if let Some(c) = i.password.chars().nth((p - 1) as usize) {
				if c == i.character {
					count += 1;
				}
			}
		};

		check_pos(i.min);
		check_pos(i.max);

		if count == 1 {
			valid_input += 1;
		}
	}

	Ok(valid_input)
}
