use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub enum Entry {
	Mask(u64, u64, u64),
	Mem(u64, u64),
}

fn parse_data(input_data: &str) -> Result<Vec<Entry>, std::io::Error> {
	Ok(input_data
		.lines()
		.map(|l| {
			let spl: Vec<&str> = l.split(" = ").collect();
			if spl[0] == "mask" {
				let (mut and_value, mut or_value, mut x_value) = (0, 0, 0);
				spl[1].chars().for_each(|c| {
					and_value <<= 1;
					or_value <<= 1;
					x_value <<= 1;
					match c {
						'X' => {
							and_value += 1;
							x_value += 1;
						}
						'0' => {}
						'1' => {
							and_value += 1;
							or_value += 1;
						}
						_ => {}
					}
				});
				Entry::Mask(and_value, or_value, x_value)
			} else {
				let mem_loc = spl[0][4..(spl[0].len() - 1)].parse().unwrap();
				let mem_value = spl[1].parse().unwrap();
				Entry::Mem(mem_loc, mem_value)
			}
		})
		.collect())
}

pub fn parse_input() -> Result<Vec<Entry>, std::io::Error> {
	let mut file = File::open("input_14.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &Vec<Entry>) -> Result<u64, std::io::Error> {
	let (mut cur_and_mask, mut cur_or_mask) = (0, 0);
	let mut memory: HashMap<u64, u64> = HashMap::new();

	input.iter().for_each(|e| match e {
		Entry::Mask(and_mask, or_mask, _) => {
			cur_and_mask = *and_mask;
			cur_or_mask = *or_mask;
		}
		Entry::Mem(mem_loc, mem_value) => {
			let final_value = (*mem_value & cur_and_mask) | cur_or_mask;
			memory.insert(*mem_loc, final_value);
		}
	});

	let sum: u64 = memory.values().sum();

	Ok(sum)
}

fn generate_variants(mask: u64, shift: u64, cur_value: u64, vec: &mut Vec<u64>) {
	if (mask >> shift) == 0 {
		vec.push(cur_value);
		return;
	}

	let mut bit_mask = 1;
	bit_mask <<= shift;
	let mut new_shift = shift;
	while mask & bit_mask == 0 {
		bit_mask <<= 1;
		new_shift += 1;
	}

	generate_variants(mask, new_shift + 1, cur_value, vec);
	generate_variants(mask, new_shift + 1, cur_value | bit_mask, vec);
}

pub fn run_b(input: &Vec<Entry>) -> Result<u64, std::io::Error> {
	let (mut cur_or_mask, mut cur_x_mask) = (0, 0);
	let mut variants: Vec<u64> = Vec::new();
	let mut memory: HashMap<u64, u64> = HashMap::new();

	input.iter().for_each(|e| match e {
		Entry::Mask(_, or_mask, x_mask) => {
			cur_or_mask = *or_mask;
			cur_x_mask = *x_mask;
			variants.clear();
			generate_variants(cur_x_mask, 0, 0, &mut variants);
		}
		Entry::Mem(mem_loc, mem_value) => {
			let masked_value = (*mem_loc & !cur_x_mask) | cur_or_mask;
			variants.iter().for_each(|v| {
				memory.insert(masked_value | v, *mem_value);
			});
		}
	});

	let sum: u64 = memory.values().sum();

	Ok(sum)
}

#[test]
fn test_aoc14() {
	let sample_input =
		"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
	let sample_res = parse_data(&sample_input).unwrap();

	let sample_input_2 = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
	let sample_res_2 = parse_data(&sample_input_2).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(165));
	assert_eq!(run_b(&sample_res_2).ok(), Some(208));
}
