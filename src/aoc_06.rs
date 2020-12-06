use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct Group {
	num_members: usize,
	yes_map: HashMap<char, usize>,
}

impl Group {
	fn new() -> Group {
		Group {
			num_members: 0,
			yes_map: HashMap::new(),
		}
	}
}

fn parse_data(input_data: &str) -> Vec<Group> {
	let mut output: Vec<Group> = Vec::new();

	output.push(Group::new());

	for l in input_data.lines() {
		if l.is_empty() {
			output.push(Group::new());
			continue;
		}

		let cur_group = output.last_mut().unwrap();
		cur_group.num_members += 1;
		l.chars().for_each(|c| {
			let entry = cur_group.yes_map.entry(c).or_insert(0);
			*entry += 1;
		});
	}

	output
}

pub fn parse_input() -> Result<Vec<Group>, std::io::Error> {
	let mut file = File::open("input_06.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	Ok(parse_data(&buf_file))
}

pub fn run_a(input: &Vec<Group>) -> Result<i64, std::io::Error> {
	let res: i64 = input.iter().map(|g| g.yes_map.len() as i64).sum();

	Ok(res)
}

pub fn run_b(input: &Vec<Group>) -> Result<i64, std::io::Error> {
	let res: i64 = input
		.iter()
		.map(|g| g.yes_map.values().filter(|c| **c == g.num_members).count())
		.sum::<usize>() as i64;

	Ok(res)
}

#[test]
fn test_aoc06() {
	let sample_input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
	let sample_res = parse_data(&sample_input);

	assert_eq!(run_a(&sample_res).ok(), Some(11));
	assert_eq!(run_b(&sample_res).ok(), Some(6));
}
