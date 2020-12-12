use std::fs::File;
use std::io::Read;

#[derive(Clone, PartialEq)]
enum Tile {
	Empty(),
	Floor(),
	Occupied(),
}

#[derive(Clone, PartialEq)]
pub struct SeatMap {
	map: Vec<Vec<Tile>>,
	width: usize,
	height: usize,
}

impl SeatMap {
	fn print(&self) {
		for y in 0..self.height {
			let mut line = String::new();
			for x in 0..self.width {
				match self.map[y][x] {
					Tile::Empty() => line.push('L'),
					Tile::Floor() => line.push('.'),
					Tile::Occupied() => line.push('#'),
				}
			}
			println!("{}", line);
		}
	}

	fn iter(&self) -> SeatMap {
		let mut new_map = (*self).clone();
		let is_occupied = |x: i64, y: i64, dir_x: i64, dir_y: i64| -> bool {
			if x + dir_x < 0
				|| x + dir_x >= self.width as i64
				|| y + dir_y < 0 || y + dir_y >= self.height as i64
			{
				return false;
			}
			match self.map[(y + dir_y) as usize][(x + dir_x) as usize] {
				Tile::Occupied() => true,
				_ => false,
			}
		};
		let num_adj = |x, y| -> i64 {
			let mut res = 0;
			for dy in -1..2 {
				for dx in -1..2 {
					if dx == 0 && dy == 0 {
						continue;
					}
					if is_occupied(x, y, dx, dy) {
						res += 1;
					}
				}
			}
			res
		};
		for y in 0..self.height {
			for x in 0..self.width {
				match self.map[y][x] {
					Tile::Empty() => {
						if num_adj(x as i64, y as i64) == 0 {
							new_map.map[y][x] = Tile::Occupied();
						}
					}
					Tile::Occupied() => {
						if num_adj(x as i64, y as i64) >= 4 {
							new_map.map[y][x] = Tile::Empty();
						}
					}
					_ => {}
				}
			}
		}
		new_map
	}

	fn iter_b(&self) -> SeatMap {
		let mut new_map = (*self).clone();
		let is_occupied = |x: i64, y: i64, dir_x: i64, dir_y: i64| -> bool {
			let mut mul = 1;
			loop {
				let ox = dir_x * mul;
				let oy = dir_y * mul;
				if x + ox < 0
					|| x + ox >= self.width as i64
					|| y + oy < 0
					|| y + oy >= self.height as i64
				{
					return false;
				}
				match self.map[(y + oy) as usize][(x + ox) as usize] {
					Tile::Occupied() => return true,
					Tile::Empty() => return false,
					_ => mul += 1,
				}
			}
		};
		let num_adj = |x, y| -> i64 {
			let mut res = 0;
			for dy in -1..2 {
				for dx in -1..2 {
					if dx == 0 && dy == 0 {
						continue;
					}
					if is_occupied(x, y, dx, dy) {
						res += 1;
					}
				}
			}
			res
		};
		for y in 0..self.height {
			for x in 0..self.width {
				match self.map[y][x] {
					Tile::Empty() => {
						if num_adj(x as i64, y as i64) == 0 {
							new_map.map[y][x] = Tile::Occupied();
						}
					}
					Tile::Occupied() => {
						if num_adj(x as i64, y as i64) >= 5 {
							new_map.map[y][x] = Tile::Empty();
						}
					}
					_ => {}
				}
			}
		}
		new_map
	}
}

fn parse_data(input_data: &str) -> Result<SeatMap, std::io::Error> {
	let map = input_data
		.lines()
		.map(|l| {
			l.chars()
				.map(|c| match c {
					'L' => Ok(Tile::Empty()),
					'.' => Ok(Tile::Floor()),
					'#' => Ok(Tile::Occupied()),
					_ => Err(()),
				})
				.collect()
		})
		.collect::<Result<Vec<Vec<Tile>>, _>>()
		.map_err(|_| {
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Unknown character on the map!",
			)
		})?;

	let width = map[0].len();
	let height = map.len();

	Ok(SeatMap { map, width, height })
}

pub fn parse_input() -> Result<SeatMap, std::io::Error> {
	let mut file = File::open("input_11.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &SeatMap) -> Result<i64, std::io::Error> {
	let mut old_map = (*input).clone();

	loop {
		let new_map = old_map.iter();

		if new_map == old_map {
			break;
		}

		old_map = new_map;
	}

	let occupied = old_map
		.map
		.iter()
		.map(|sv| {
			sv.iter()
				.map(|v| if *v == Tile::Occupied() { 1 } else { 0 })
				.sum::<i64>()
		})
		.sum();

	Ok(occupied)
}

pub fn run_b(input: &SeatMap) -> Result<i64, std::io::Error> {
	let mut old_map = (*input).clone();

	loop {
		let new_map = old_map.iter_b();

		if new_map == old_map {
			break;
		}

		old_map = new_map;
	}

	let occupied = old_map
		.map
		.iter()
		.map(|sv| {
			sv.iter()
				.map(|v| if *v == Tile::Occupied() { 1 } else { 0 })
				.sum::<i64>()
		})
		.sum();

	Ok(occupied)
}

#[test]
fn test_aoc11() {
	let sample_input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
	let sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(37));
	assert_eq!(run_b(&sample_res).ok(), Some(26));
}
