use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Clone, Debug)]
pub enum Data {
	Number(u64),
	SubExpr(Vec<Data>),
	Plus(),
	Mul(),
}

type PuzzleData = Vec<Vec<Data>>;

fn parse_expression(line: &str) -> Vec<Data> {
	let mut res = Vec::new();
	let mut cur_number = String::new();
	let mut depth = 0;

	let process_number = |num_str: &mut String, to_add: &mut Vec<Data>| {
		if !num_str.is_empty() {
			to_add.push(Data::Number(num_str.parse().unwrap()));
			num_str.clear();
		}
	};

	for i in 0..line.len() {
		if depth < 0 {
			break;
		}

		let c = line.chars().nth(i).unwrap();

		if depth > 0 {
			match c {
				'(' => depth += 1,
				')' => depth -= 1,
				_ => {}
			}
			continue;
		}

		match c {
			'0'..='9' => cur_number.push(c),
			' ' => process_number(&mut cur_number, &mut res),
			'(' => {
				depth += 1;
				res.push(Data::SubExpr(parse_expression(&line[(i + 1)..])));
			}
			')' => {
				depth -= 1;
				process_number(&mut cur_number, &mut res);
			}
			'+' => res.push(Data::Plus()),
			'*' => res.push(Data::Mul()),
			_ => panic!("Unknown char: {}!", c),
		}
	}

	process_number(&mut cur_number, &mut res);

	res
}

fn parse_data(input_data: &str) -> Result<PuzzleData, std::io::Error> {
	Ok(input_data.lines().map(|l| parse_expression(l)).collect())
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_18.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

fn evaluate(data: &Vec<Data>) -> u64 {
	let mut cur_value = 0;
	let mut cur_op = Data::Plus();

	let mut apply_op = |op: &Data, value: u64| {
		match op {
			Data::Plus() => cur_value += value,
			Data::Mul() => cur_value *= value,
			_ => panic!("Invalid op!"),
		}
	};

	data.iter().for_each(|d| {
		match d {
			Data::Number(v) => apply_op(&cur_op, *v),
			Data::SubExpr(v) => apply_op(&cur_op, evaluate(v)),
			Data::Plus() => cur_op = Data::Plus(),
			Data::Mul() => cur_op = Data::Mul(),
		}
	});

	cur_value
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	Ok(input.iter().map(|e| evaluate(e)).sum())
}

fn regroup_plus(data: &Vec<Data>) -> Vec<Data> {
	let mut new_vec = Vec::new();
	let mut i = 0;

	let regroup = |sub: &Data| -> Data {
		if let Data::SubExpr(s) = sub {
			Data::SubExpr(regroup_plus(s))
		} else {
			sub.clone()
		}
	};

	while i < (data.len() - 1) {
		if data[i + 1] == Data::Plus() && data.len() != 3 {
			let mut sub_vec = Vec::new();
			sub_vec.push(regroup(&data[i]));
			sub_vec.push(regroup(&data[i+1]));
			sub_vec.push(regroup(&data[i+2]));
			new_vec.push(Data::SubExpr(sub_vec));
			for si in (i+3)..data.len() {
				new_vec.push(data[si].clone());
			}
			return regroup_plus(&new_vec);
		} else {
			new_vec.push(regroup(&data[i]));
			i += 1;
		}
	}

	if i < data.len() {
		new_vec.push(regroup(&data[i]));
	}

	new_vec
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let reordered: PuzzleData = input.iter().map(|d| regroup_plus(d)).collect();
	Ok(reordered.iter().map(|e| evaluate(e)).sum())
}

#[test]
fn test_aoc18() {
	let sample_res_1 = parse_data(&"2 * 3 + (4 * 5)").unwrap();
	let sample_res_2 = parse_data(&"5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();
	let sample_res_3 = parse_data(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
	let sample_res_4 = parse_data(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();

	assert_eq!(run_a(&sample_res_1).ok(), Some(26));
	assert_eq!(run_a(&sample_res_2).ok(), Some(437));
	assert_eq!(run_a(&sample_res_3).ok(), Some(12240));
	assert_eq!(run_a(&sample_res_4).ok(), Some(13632));
	assert_eq!(run_b(&sample_res_1).ok(), Some(46));
	assert_eq!(run_b(&sample_res_2).ok(), Some(1445));
	assert_eq!(run_b(&sample_res_3).ok(), Some(669060));
	assert_eq!(run_b(&sample_res_4).ok(), Some(23340));
}
