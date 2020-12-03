use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum TileType {
	Empty,
	Tree,
}

struct InputMap {
	map: Vec<Vec<TileType>>,
	width: usize,
	height: usize,
}

fn parse_input() -> Result<InputMap, std::io::Error> {
	let file = File::open("input_03.txt")?;

	let map: Vec<Vec<TileType>> = BufReader::new(file)
		.lines()
		.map(|line| -> Result<Vec<TileType>, ()> {
			line.unwrap()
				.chars()
				.map(|c| match c {
					'.' => Ok(TileType::Empty),
					'#' => Ok(TileType::Tree),
					_ => Err(()),
				})
				.collect()
		})
		.collect::<Result<Vec<Vec<TileType>>, _>>()
		.map_err(|_| {
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Invalid character on the map!",
			)
		})?;

	let width = map[0].len();
	let height = map.len();

	Ok(InputMap { map, width, height })
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let (mut x, mut y) : (usize, usize)= (0, 0);
	let mut num_trees = 0;

	while y < input.height {
		if input.map[y][x] == TileType::Tree {
			num_trees += 1;
		}

		x += 3;
		y += 1;
		x %= input.width;
	}

	Ok(num_trees)
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let get_num_trees = |slope_x: usize, slope_y: usize| -> i64 {
		let (mut x, mut y) : (usize, usize)= (0, 0);
		let mut num_trees = 0;
	
		while y < input.height {
			if input.map[y][x] == TileType::Tree {
				num_trees += 1;
			}
	
			x += slope_x;
			y += slope_y;
			x %= input.width;
		}

		num_trees
	};

	Ok(get_num_trees(1, 1) * get_num_trees(3, 1) * get_num_trees(5, 1) * get_num_trees(7, 1) * get_num_trees(1, 2))
}
