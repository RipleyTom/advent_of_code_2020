use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Passport {
	inputs: HashMap<String, String>,
}

fn parse_input() -> Result<Vec<Passport>, std::io::Error> {
	let file = File::open("input_04.txt")?;
	let buf_file = BufReader::new(file);
	let mut output: Vec<Passport> = Vec::new();

	output.push(Passport {
		inputs: HashMap::new(),
	});

	for l in buf_file.lines() {
		let l = l.unwrap();
		if l.is_empty() {
			output.push(Passport {
				inputs: HashMap::new(),
			});
			continue;
		}

		let cur_pass = output.last_mut().unwrap();
		let new_hashmap: HashMap<String, String> = l
			.split(' ')
			.collect::<Vec<&str>>()
			.iter()
			.map(|e| {
				let res: Vec<&str> = e.split(':').collect();
				if res.len() == 2 {
					Ok((String::from(res[0]), String::from(res[1])))
				} else {
					Err(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"Invalid character on the map!",
					))
				}
			})
			.collect::<Result<HashMap<String, String>, std::io::Error>>()?;

		cur_pass.inputs.extend(new_hashmap);
	}

	Ok(output)
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;
	let required_inputs_str: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let required_inputs: Vec<String> = required_inputs_str.iter().map(|s| s.to_string()).collect();

	let valid_inputs = input
		.iter()
		.map(|p| {
			required_inputs
				.iter()
				.map(|r| p.inputs.contains_key(r))
				.collect::<Vec<bool>>()
				.iter()
				.all(|r| *r == true)
		})
		.filter(|a| *a == true)
		.count() as i64;

	Ok(valid_inputs)
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;
	let required_inputs_str: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let required_inputs: Vec<String> = required_inputs_str.iter().map(|s| s.to_string()).collect();

	// Should use regexp here but ~eh

	let validate_year = |i: &String, miny: i64, maxy: i64| -> bool {
		let parsed: Result<i64, _> = i.parse();
		if i.len() != 4 || parsed.is_err() {
			return false;
		}
		let parsed = parsed.unwrap();

		if parsed < miny || parsed > maxy {
			return false;
		}
		true
	};

	let validate_height = |i: &String| -> bool {
		if i.len() != 4 && i.len() != 5 {
			return false;
		}

		let (n, m) = i.split_at(i.len() - 2);
		let n_parsed: Result<i64, _> = n.parse();
		if n_parsed.is_err() {
			return false;
		}
		let n_parsed = n_parsed.unwrap();
		match m {
			"cm" => {
				if n_parsed < 150 || n_parsed > 193 {
					return false;
				}
			}
			"in" => {
				if n_parsed < 59 || n_parsed > 76 {
					return false;
				}
			}
			_ => {
				return false;
			}
		}

		true
	};

	let validate_hair = |i: &String| -> bool {
		if i.len() != 7 || i.chars().nth(0).unwrap() != '#' {
			return false;
		}

		for x in 1..7 {
			let c = i.chars().nth(x).unwrap();
			if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
				return false;
			}
		}

		true
	};

	let validate_eye = |i: &String| -> bool {
		let valid_eyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
		valid_eyes.iter().any(|ecl| ecl == i)
	};

	let validate_pid = |i: &String| -> bool {
		if i.len() != 9 || !i.chars().all(|c| c.is_ascii_digit()) {
			return false;
		}
		true
	};

	let valid_inputs = input
		.iter()
		.map(|p| {
			if required_inputs
				.iter()
				.map(|r| p.inputs.contains_key(r))
				.collect::<Vec<bool>>()
				.iter()
				.all(|r| *r == true)
				== false
			{
				return false;
			}

			validate_year(&p.inputs["byr"], 1920, 2002)
				&& validate_year(&p.inputs["iyr"], 2010, 2020)
				&& validate_year(&p.inputs["eyr"], 2020, 2030)
				&& validate_height(&p.inputs["hgt"])
				&& validate_hair(&p.inputs["hcl"])
				&& validate_eye(&p.inputs["ecl"])
				&& validate_pid(&p.inputs["pid"])
		})
		.filter(|a| *a == true)
		.count() as i64;

	Ok(valid_inputs)
}
