use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Clone, Debug)]
pub enum Rule {
	Single(u64),
	Duo(u64, u64),
	Trio(u64, u64, u64),
	Char(char),
}

pub struct PuzzleData {
	rules: HashMap<u64, Vec<Rule>>,
	data: Vec<String>,
}

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	enum ParseState {
		Rule,
		Data,
	}

	let mut cur_state = ParseState::Rule;
	let mut rules: HashMap<u64, Vec<Rule>> = HashMap::new();
	let mut data: Vec<String> = Vec::new();

	input.lines().for_each(|l| {
		if l.is_empty() {
			cur_state = ParseState::Data;
			return;
		}

		match cur_state {
			ParseState::Rule => {
				let num_rules: Vec<&str> = l.split(": ").collect();
				let num: u64 = num_rules[0].parse().unwrap();
				let list_rules: Vec<Rule> = num_rules[1]
					.split(" | ")
					.map(|r| {
						let subsets: Vec<&str> = r.split(' ').collect();
						match subsets.len() {
							1 => {
								if subsets[0].chars().nth(0).unwrap() == '\"' {
									Rule::Char(subsets[0].chars().nth(1).unwrap())
								} else {
									Rule::Single(subsets[0].parse().unwrap())
								}
							}
							2 => Rule::Duo(subsets[0].parse().unwrap(), subsets[1].parse().unwrap()),
							3 => Rule::Trio(subsets[0].parse().unwrap(), subsets[1].parse().unwrap(), subsets[2].parse().unwrap()),
							u => panic!("Unexpected amount of elements in the sub rule: {} ({})!", u, l),
						}
					})
					.collect();
				rules.insert(num, list_rules);
			}
			ParseState::Data => {
				data.push(String::from(l));
			}
		}
	});

	Ok(PuzzleData { rules, data })
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_19.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

fn is_match(data: &str, rset: &HashMap<u64, Vec<Rule>>, rule: u64, si: usize) -> Vec<usize> {
	let mut vec_index = Vec::new();

	rset[&rule].iter().for_each(|r| {
		match r {
			Rule::Char(c) => {
				match data.chars().nth(si) {
					Some(ch) if *c == ch => vec_index.push(si + 1),
					_ => {},
				}
			},
			Rule::Single(sr) => {
				vec_index.extend(is_match(data, rset, *sr, si));
			}
			Rule::Duo(sr, sr2) => {
				let poss_index = is_match(data, rset, *sr, si);
				for i in &poss_index {
					vec_index.extend(is_match(data, rset, *sr2, *i));
				}
			}
			Rule::Trio(sr, sr2, sr3) => {
				let poss_index = is_match(data, rset, *sr, si);
				for i in &poss_index {
					let poss2_index = is_match(data, rset, *sr2, *i);
					for i2 in &poss2_index {
						vec_index.extend(is_match(data, rset, *sr3, *i2));	
					}
				}

			}
		}
	});

	vec_index
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut num_valid = 0;

	for data in &input.data {
		if is_match(data, &input.rules, 0, 0).iter().any(|i| *i == data.len()) {
			num_valid += 1;
		}

	}

	Ok(num_valid)
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut new_rules = input.rules.clone();
	new_rules.insert(8, vec![Rule::Single(42), Rule::Duo(42, 8)]);
	new_rules.insert(11, vec![Rule::Duo(42, 31), Rule::Trio(42, 11, 31)]);

	let mut num_valid = 0;

	for data in &input.data {
		if is_match(data, &new_rules, 0, 0).iter().any(|i| *i == data.len()) {
			num_valid += 1;
		}
	}

	Ok(num_valid)
}

#[test]
fn test_aoc19() {
	let sample_res = parse_data(&"0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb").unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(2));
}
