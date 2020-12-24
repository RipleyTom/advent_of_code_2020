use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub enum Direction {
	East,
	SouthEast,
	SouthWest,
	West,
	NorthWest,
	NorthEast,
}

type PuzzleData = Vec<Vec<Direction>>;

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	let mut res: PuzzleData = Vec::new();
	for l in input.lines() {
		let mut cur_dir = Vec::new();
		let mut duo = false;
		let mut north = false;

		for c in l.chars() {
			let is_duo = duo;
			duo = false;
			match c {
				'e' if is_duo && north => cur_dir.push(Direction::NorthEast),
				'e' if is_duo && !north => cur_dir.push(Direction::SouthEast),
				'e' if !is_duo => cur_dir.push(Direction::East),
				'w' if is_duo && north => cur_dir.push(Direction::NorthWest),
				'w' if is_duo && !north => cur_dir.push(Direction::SouthWest),
				'w' if !is_duo => cur_dir.push(Direction::West),
				'n' => {
					duo = true;
					north = true;
				},
				's' => {
					duo = true;
					north = false;
				},
				_ => panic!("Unexpected char in input!"),
			}
		}
		res.push(cur_dir);
	}
	Ok(res)
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_24.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut tile_info: HashMap<(i32, i32), bool> = HashMap::new();

	for directions in input {
		let (mut x, mut y) = (0, 0);
		for d in directions {
			match d {
				Direction::East => x += 2,
				Direction::SouthEast => {
					x+=1;
					y+=1;
				},
				Direction::SouthWest => {
					x-=1;
					y+=1;
				},
				Direction::West => x -= 2,
				Direction::NorthWest => {
					x -= 1;
					y -= 1;
				},
				Direction::NorthEast => {
					x += 1;
					y -= 1;
				},
			}
		}
		let ent = tile_info.entry((x, y)).or_insert(false);
		*ent = !*ent;
	}

	let res = tile_info.iter().map(|(_, v)| if *v { 1 } else { 0 }).sum();

	Ok(res)
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut tile_info: HashMap<(i32, i32), bool> = HashMap::new();

	for directions in input {
		let (mut x, mut y) = (0, 0);
		for d in directions {
			match d {
				Direction::East => x += 2,
				Direction::SouthEast => {
					x+=1;
					y+=1;
				},
				Direction::SouthWest => {
					x-=1;
					y+=1;
				},
				Direction::West => x -= 2,
				Direction::NorthWest => {
					x -= 1;
					y -= 1;
				},
				Direction::NorthEast => {
					x += 1;
					y -= 1;
				},
			}
		}
		let ent = tile_info.entry((x, y)).or_insert(false);
		*ent = !*ent;
	}

	let dirs = [(2, 0), (1, 1), (-1, 1), (-2, 0), (-1, -1), (1, -1)];

	for _ in 0..100 {
		let mut new_tiles: HashMap<(i32, i32), bool> = HashMap::new();
		let mut pot_switches: HashMap<(i32, i32), u8> = HashMap::new();

		for ((x, y), v) in &tile_info {
			if !v {
				continue;
			}

			let mut num_black = 0;

			for (dx, dy) in &dirs {
				if let Some(b) = tile_info.get(&(x+dx, y+dy)) {
					if *b {
						num_black += 1;
					} else {
						let ent = pot_switches.entry((x+dx, y+dy)).or_insert(0);
						*ent += 1;
					}
				} else {
					let ent = pot_switches.entry((x+dx, y+dy)).or_insert(0);
					*ent += 1;
				}
			}

			if num_black == 1 || num_black == 2 {
				new_tiles.insert((*x, *y), true);
			}
		}

		for ((x, y), sn) in &pot_switches {
			if *sn == 2 {
				new_tiles.insert((*x, *y), true);
			}
		}

		tile_info = new_tiles;
	}

	let res = tile_info.iter().map(|(_, v)| if *v { 1 } else { 0 }).sum();

	Ok(res)
}

#[test]
fn test_aoc24() {
	let sample_res = parse_data(&"sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew").unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(10));
	assert_eq!(run_b(&sample_res).ok(), Some(2208));
}
