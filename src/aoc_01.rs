use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<i64>, std::io::Error> {
	let file = File::open("input_01.txt")?;

	let result: Vec<i64> = BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().trim().parse().unwrap())
		.collect();

	Ok(result)
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let mut result: Option<i64> = None;

	'main_loop: for i in 0..input.len() {
		for j in (i + 1)..input.len() {
			if input[i] + input[j] == 2020 {
				result = Some(input[i] * input[j]);
				break 'main_loop;
			}
		}
	}

	if result.is_some() {
		Ok(result.unwrap())
	} else {
		Err(std::io::Error::new(
			std::io::ErrorKind::InvalidData,
			"Couldn't find a 2020 match!",
		))
	}
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let mut result: Option<i64> = None;

	'main_loop: for i in 0..input.len() {
		for j in (i + 1)..input.len() {
			for k in (j + 1)..input.len() {
				if input[i] + input[j] + input[k] == 2020 {
					result = Some(input[i] * input[j] * input[k]);
					break 'main_loop;
				}
			}
		}
	}

	if result.is_some() {
		Ok(result.unwrap())
	} else {
		Err(std::io::Error::new(
			std::io::ErrorKind::InvalidData,
			"Couldn't find a 2020 match!",
		))
	}
}
