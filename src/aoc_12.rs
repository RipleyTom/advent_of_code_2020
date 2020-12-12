use std::fs::File;
use std::io::Read;

pub enum Inst {
	N(i64),
	S(i64),
	E(i64),
	W(i64),
	L(i64),
	R(i64),
	F(i64),
}

fn parse_data(input_data: &str) -> Result<Vec<Inst>, std::io::Error> {
	input_data
		.lines()
		.map(|l| {
			let num = l[1..].parse().unwrap();
			match l.chars().nth(0).unwrap() {
				'N' => Ok(Inst::N(num)),
				'S' => Ok(Inst::S(num)),
				'E' => Ok(Inst::E(num)),
				'W' => Ok(Inst::W(num)),
				'L' => Ok(Inst::L(num)),
				'R' => Ok(Inst::R(num)),
				'F' => Ok(Inst::F(num)),
				_ => Err(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					"Unknown instruction found!",
				)),
			}
		})
		.collect()
}

pub fn parse_input() -> Result<Vec<Inst>, std::io::Error> {
	let mut file = File::open("input_12.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &Vec<Inst>) -> Result<i64, std::io::Error> {
	let (mut cur_x, mut cur_y) = (0, 0);
	let mut dir = 0;

	input.iter().for_each(|i| match i {
		Inst::N(n) => cur_y -= n,
		Inst::S(n) => cur_y += n,
		Inst::E(n) => cur_x += n,
		Inst::W(n) => cur_x -= n,
		Inst::L(n) => dir -= n,
		Inst::R(n) => dir += n,
		Inst::F(n) => match (dir % 360) / 90 {
			0 | -4 => cur_x += n,
			1 | -3 => cur_y += n,
			2 | -2 => cur_x -= n,
			3 | -1 => cur_y -= n,
			_ => {}
		},
	});

	Ok(cur_x.abs() + cur_y.abs())
}

pub fn run_b(input: &Vec<Inst>) -> Result<i64, std::io::Error> {
	let (mut way_x, mut way_y) = (10, -1);
	let (mut cur_x, mut cur_y) = (0, 0);

	let rotate = |r, cx: &mut i64, cy: &mut i64| {
		let num_rot = {
			let res = (r % 360) / 90;
			if res < 0 {
				4 + res
			} else {
				res
			}
		};

		(0..num_rot).for_each(|_| {
			let tmp_x = *cx;
			*cx = -(*cy);
			*cy = tmp_x;
		});
	};

	input.iter().for_each(|i| match i {
		Inst::N(n) => way_y -= n,
		Inst::S(n) => way_y += n,
		Inst::E(n) => way_x += n,
		Inst::W(n) => way_x -= n,
		Inst::L(n) => rotate(-n, &mut way_x, &mut way_y),
		Inst::R(n) => rotate(*n, &mut way_x, &mut way_y),
		Inst::F(n) => {
			cur_x += n * way_x;
			cur_y += n * way_y;
		}
	});

	Ok(cur_x.abs() + cur_y.abs())
}

#[test]
fn test_aoc12() {
	let sample_input = "F10\nN3\nF7\nR90\nF11";
	let sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(25));
	assert_eq!(run_b(&sample_res).ok(), Some(286));
}
