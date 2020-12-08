use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
enum Instruction {
	Acc(i64),
	Jmp(i64),
	Nop(i64),
}

pub struct Program {
	data: Vec<Instruction>,
	offset: i64,
	acc: i64,
}

impl Program {
	fn reset_state(&mut self) {
		self.offset = 0;
		self.acc = 0;
	}

	fn run_instruction(&mut self) {
		match self.data[self.offset as usize] {
			Instruction::Acc(v) => {
				self.acc += v;
				self.offset += 1;
			}
			Instruction::Jmp(v) => self.offset += v,
			Instruction::Nop(_) => self.offset += 1,
		}
	}
}

fn parse_data(input_data: &str) -> Result<Program, std::io::Error> {
	let data = input_data
		.lines()
		.map(|l| {
			let ins_val: Vec<&str> = l.split(' ').collect();
			let val = ins_val[1].parse().unwrap();
			match ins_val[0] {
				"acc" => Ok(Instruction::Acc(val)),
				"jmp" => Ok(Instruction::Jmp(val)),
				"nop" => Ok(Instruction::Nop(val)),
				_ => Err(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					"Invalid instruction in the program!",
				)),
			}
		})
		.collect::<Result<Vec<Instruction>, _>>()?;

	Ok(Program {
		data,
		offset: 0,
		acc: 0,
	})
}

pub fn parse_input() -> Result<Program, std::io::Error> {
	let mut file = File::open("input_08.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &mut Program) -> Result<i64, std::io::Error> {
	input.reset_state();
	let mut seen_offsets: HashSet<i64> = HashSet::new();

	while !seen_offsets.contains(&input.offset) {
		seen_offsets.insert(input.offset);
		input.run_instruction();
	}

	Ok(input.acc)
}

fn test_program(input: &mut Program, mut seen_offsets: HashSet<i64>) -> Option<i64> {
	let len_program = input.data.len() as i64;

	while input.offset != len_program {
		if seen_offsets.contains(&input.offset) {
			return None;
		}
		seen_offsets.insert(input.offset);
		input.run_instruction();
	}

	Some(input.acc)
}

fn swap_and_test(
	input: &mut Program,
	seen_offsets: &HashSet<i64>,
	inst: Instruction,
) -> Option<i64> {
	let (save_off, save_acc, save_ins) = (
		input.offset,
		input.acc,
		input.data[input.offset as usize].clone(),
	);
	input.data[input.offset as usize] = inst;
	if let Some(r) = test_program(input, seen_offsets.clone()) {
		return Some(r);
	}
	input.offset = save_off;
	input.acc = save_acc;
	input.data[input.offset as usize] = save_ins;

	None
}

pub fn run_b(input: &mut Program) -> Result<i64, std::io::Error> {
	input.reset_state();
	let mut seen_offsets: HashSet<i64> = HashSet::new();

	let len_program = input.data.len() as i64;

	while input.offset != len_program {
		match input.data[input.offset as usize] {
			Instruction::Jmp(v) => {
				if let Some(r) = swap_and_test(input, &seen_offsets, Instruction::Nop(v)) {
					return Ok(r);
				}
			}
			Instruction::Nop(v) => {
				if let Some(r) = swap_and_test(input, &seen_offsets, Instruction::Jmp(v)) {
					return Ok(r);
				}
			}
			_ => {}
		}
		seen_offsets.insert(input.offset);
		input.run_instruction();
	}

	Ok(input.acc)
}

#[test]
fn test_aoc08() {
	let sample_input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6\n";
	let mut sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&mut sample_res).ok(), Some(5));
	assert_eq!(run_b(&mut sample_res).ok(), Some(8));
}
