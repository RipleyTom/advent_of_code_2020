use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub struct PuzzleData {
	init_x: u64,
	init_y: u64,
	initial_map: Vec<Vec<bool>>,
}

fn parse_data(input_data: &str) -> Result<PuzzleData, std::io::Error> {
	let initial_map: Vec<Vec<bool>> = input_data
		.lines()
		.map(|l| {
			l.chars()
				.map(|c| match c {
					'#' => Ok(true),
					'.' => Ok(false),
					_ => Err(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"Unknown instruction found!",
					)),
				})
				.collect()
		})
		.collect::<Result<Vec<Vec<bool>>, std::io::Error>>()?;

	Ok(PuzzleData {
		init_x: initial_map[0].len() as u64,
		init_y: initial_map.len() as u64,
		initial_map,
	})
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_17.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut list_active: HashSet<u64> = HashSet::new();

	let get_x_y_z = |coord: u64| -> (i64, i64, i64) {
		let x = (coord >> 42) & 0x1FFFFF;
		let y = (coord >> 21) & 0x1FFFFF;
		let z = coord & 0x1FFFFF;
		(x as i64, y as i64, z as i64)
	};

	let gen_coord = |x: u64, y: u64, z: u64| -> u64 { (x << 42) | (y << 21) | z };

	let final_x = input.init_x + 13;
	let final_y = input.init_y + 13;

	for y in 0..input.init_y {
		for x in 0..input.init_x {
			if input.initial_map[y as usize][x as usize] == true {
				list_active.insert(gen_coord(x + (final_x / 2), y + (final_y / 2), 6));
			}
		}
	}

	for _ in 0..6 {
		let mut new_active: HashSet<u64> = HashSet::new();
		let mut to_check: HashSet<u64> = HashSet::new();

		for a in &list_active {
			let (cx, cy, cz) = get_x_y_z(*a);
			let mut num_active = 0;

			for x in -1..2 {
				for y in -1..2 {
					for z in -1..2 {
						if x == 0 && y == 0 && z == 0 {
							continue;
						}
						let coord = gen_coord((cx + x) as u64, (cy + y) as u64, (cz + z) as u64);
						if list_active.contains(&coord) {
							num_active += 1;
						} else {
							to_check.insert(coord);
						}
					}
				}
			}

			if num_active == 2 || num_active == 3 {
				new_active.insert(*a);
			}
		}

		for a in &to_check {
			let (cx, cy, cz) = get_x_y_z(*a);
			let mut num_active = 0;
			for x in -1..2 {
				for y in -1..2 {
					for z in -1..2 {
						if x == 0 && y == 0 && z == 0 {
							continue;
						}
						let coord = gen_coord((cx + x) as u64, (cy + y) as u64, (cz + z) as u64);
						if list_active.contains(&coord) {
							num_active += 1;
						}
					}
				}
			}
			if num_active == 3 {
				new_active.insert(*a);
			}
		}

		list_active = new_active;
	}

	Ok(list_active.len() as u64)
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut list_active: HashSet<u64> = HashSet::new();

	let get_x_y_z_w = |coord: u64| -> (i64, i64, i64, i64) {
		let x = (coord >> 48) & 0xFFFF;
		let y = (coord >> 32) & 0xFFFF;
		let z = (coord >> 16) & 0xFFFF;
		let w = coord & 0xFFFF;
		(x as i64, y as i64, z as i64, w as i64)
	};

	let gen_coord =
		|x: u64, y: u64, z: u64, w: u64| -> u64 { (x << 48) | (y << 32) | (z << 16) | w };

	let final_x = input.init_x + 13;
	let final_y = input.init_y + 13;

	for y in 0..input.init_y {
		for x in 0..input.init_x {
			if input.initial_map[y as usize][x as usize] == true {
				list_active.insert(gen_coord(x + (final_x / 2), y + (final_y / 2), 6, 6));
			}
		}
	}

	for _ in 0..6 {
		let mut new_active: HashSet<u64> = HashSet::new();
		let mut to_check: HashSet<u64> = HashSet::new();

		for a in &list_active {
			let (cx, cy, cz, cw) = get_x_y_z_w(*a);
			let mut num_active = 0;

			for x in -1..2 {
				for y in -1..2 {
					for z in -1..2 {
						for w in -1..2 {
							if x == 0 && y == 0 && z == 0 && w == 0 {
								continue;
							}
							let coord = gen_coord(
								(cx + x) as u64,
								(cy + y) as u64,
								(cz + z) as u64,
								(cw + w) as u64,
							);
							if list_active.contains(&coord) {
								num_active += 1;
							} else {
								to_check.insert(coord);
							}
						}
					}
				}
			}

			if num_active == 2 || num_active == 3 {
				new_active.insert(*a);
			}
		}

		for a in &to_check {
			let (cx, cy, cz, cw) = get_x_y_z_w(*a);
			let mut num_active = 0;
			for x in -1..2 {
				for y in -1..2 {
					for z in -1..2 {
						for w in -1..2 {
							if x == 0 && y == 0 && z == 0 && w == 0 {
								continue;
							}
							let coord = gen_coord(
								(cx + x) as u64,
								(cy + y) as u64,
								(cz + z) as u64,
								(cw + w) as u64,
							);
							if list_active.contains(&coord) {
								num_active += 1;
							}
						}
					}
				}
			}
			if num_active == 3 {
				new_active.insert(*a);
			}
		}

		list_active = new_active;
	}

	Ok(list_active.len() as u64)
}

#[test]
fn test_aoc17() {
	let sample_input = ".#.\n..#\n###";
	let sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(112));
	assert_eq!(run_b(&sample_res).ok(), Some(848));
}
