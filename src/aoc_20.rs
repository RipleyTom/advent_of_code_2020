use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

struct Tile {
	data: Vec<Vec<bool>>,
	sides: [u16; 8],
}

impl Tile {
	fn new() -> Tile {
		Tile {
			data: Vec::new(),
			sides: [0; 8],
		}
	}
}

pub struct PuzzleData {
	tiles: HashMap<u64, Tile>,
}

fn flip_it(v: u16) -> u16 {
	let mut res = 0;
	for i in 0..10 {
		res |= ((v >> (9 - i)) & 1) << i;
	}
	res
}

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	let mut tiles = HashMap::new();
	let mut cur_tile = Tile::new();
	let mut tile_num = 0;

	let calc_side = |tile: &Tile, x: i64, y: i64, incx: i64, incy: i64| -> u16 {
		let mut res = 0;
		let mut cx = x;
		let mut cy = y;
		for _ in 0..10 {
			res <<= 1;
			if tile.data[cy as usize][cx as usize] {
				res += 1;
			}
			cx += incx;
			cy += incy;
		}
		res
	};

	let calc_sides = |tile: &mut Tile| {
		tile.sides[0] = calc_side(&tile, 0, 9, 0, -1);
		tile.sides[1] = calc_side(&tile, 0, 0, 1, 0);
		tile.sides[2] = calc_side(&tile, 9, 0, 0, 1);
		tile.sides[3] = calc_side(&tile, 9, 9, -1, 0);
		tile.sides[4] = flip_it(tile.sides[2]);
		tile.sides[5] = flip_it(tile.sides[1]);
		tile.sides[6] = flip_it(tile.sides[0]);
		tile.sides[7] = flip_it(tile.sides[3]);
	};

	for l in input.lines() {
		if l.is_empty() {
			assert_eq!(cur_tile.data.len(), 10);
			assert_eq!(cur_tile.data[0].len(), 10);
			calc_sides(&mut cur_tile);
			tiles.insert(tile_num, cur_tile);
			cur_tile = Tile::new();
			continue;
		}

		if &l[0..5] == "Tile " {
			tile_num = (&l[5..(l.len() - 1)]).parse().unwrap();
			continue;
		}

		cur_tile.data.push(
			l.chars()
				.map(|c| match c {
					'#' => true,
					'.' => false,
					_ => panic!("Unexpected char in tile data!"),
				})
				.collect(),
		);
	}

	calc_sides(&mut cur_tile);
	tiles.insert(tile_num, cur_tile);

	Ok(PuzzleData { tiles })
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_20.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut corners: Vec<u64> = Vec::new();
	for (tile_num, tile) in &input.tiles {
		let mut set: HashSet<u16> = tile.sides.iter().map(|v| *v).collect();
		for (ot_num, otile) in &input.tiles {
			if tile_num == ot_num {
				continue;
			}
			for i in 0..8 {
				set.remove(&otile.sides[i]);
			}
		}

		if set.len() == 4 {
			corners.push(*tile_num);
		}
	}

	assert_eq!(corners.len(), 4);

	Ok(corners.iter().product())
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut corners: Vec<u64> = Vec::new();
	for (tile_num, tile) in &input.tiles {
		let mut set: HashSet<u16> = tile.sides.iter().map(|v| *v).collect();
		for (ot_num, otile) in &input.tiles {
			if tile_num == ot_num {
				continue;
			}
			for i in 0..8 {
				set.remove(&otile.sides[i]);
			}
		}

		if set.len() >= 4 {
			corners.push(*tile_num);
		}
	}
	assert_eq!(corners.len(), 4);

	#[derive(Clone)]
	struct TileInfo {
		tile_id: u64,
		rotation: usize,
		flip: bool,
		sides: [u16; 4],
	}
	impl TileInfo {
		fn new() -> TileInfo {
			TileInfo {
				tile_id: 0,
				rotation: 0,
				flip: false,
				sides: [0; 4],
			}
		}
	}

	let available_tiles: HashSet<u64> = input.tiles.keys().map(|k| *k).collect();
	let size_map: usize = (available_tiles.len() as f64).sqrt() as usize;

	let mut map: Vec<Vec<TileInfo>> = Vec::new();

	for _ in 0..size_map {
		let mut cur_line = Vec::new();
		for _ in 0..size_map {
			cur_line.push(TileInfo::new());
		}
		map.push(cur_line);
	}

	let find_tile_with_side = |side: u16, avail: &HashSet<u64>| -> Result<(u64, usize), ()> {
		let mut possible: Vec<(u64, usize)> = Vec::new();

		for k in avail {
			for i in 0..8 {
				if input.tiles[k].sides[i] == side {
					possible.push((*k, i));
				}
			}
		}

		println!("Checking for {}, possible :{:?}", side, possible);
		if possible.len() == 1 {
			Ok(possible[0])
		} else {
			Err(())
		}
	};

	let print_tile = |flip: bool, rot: usize, tile: &Tile| {
		let mut tile_flip = tile.data.clone();
		if flip {
			for y in 0..10 {
				for x in 0..10 {
					tile_flip[y][x] = tile.data[y][9 - x];
				}
			}
		}

		let mut tile_rot = tile_flip.clone();
		for _ in 0..rot {
			let mut new_rot = tile_rot.clone();
			for y in 0..10 {
				for x in 0..10 {
					new_rot[x][9 - y] = tile_rot[y][x];
				}
			}
			tile_rot = new_rot;
		}

		for y in 0..10 {
			let line_str: String = tile_rot[y]
				.iter()
				.map(|b| if *b { '#' } else { '.' })
				.collect();
			println!("{}", line_str);
		}
	};

	let get_tile_data = |flip: bool, rot: usize, tile: &Tile| -> Vec<Vec<bool>> {
		let mut tile_flip = tile.data.clone();
		if flip {
			for y in 0..10 {
				for x in 0..10 {
					tile_flip[y][x] = tile.data[y][9 - x];
				}
			}
		}
		let mut tile_rot = tile_flip.clone();
		for _ in 0..rot {
			let mut new_rot = tile_rot.clone();
			for y in 0..10 {
				for x in 0..10 {
					new_rot[x][9 - y] = tile_rot[y][x];
				}
			}
			tile_rot = new_rot;
		}

		let mut final_tile: Vec<Vec<bool>> = Vec::new();
		for y in 1..9 {
			final_tile.push(Vec::new());
			final_tile[y - 1].extend(&tile_rot[y][1..9]);
		}

		final_tile
	};

	let calc_flip_rot = |flip: bool, rot: usize, tile: &Tile| -> [u16; 4] {
		let mut res = [0; 4];

		let to_add = if flip { 4 } else { 0 };

		for i in 0..4 {
			res[(i + rot) % 4] = tile.sides[to_add + i];
		}

		println!("{:?}", res);
		print_tile(flip, rot, tile);
		res
	};

	let mut check_corner = |corner_id: u64,
	                        initial_rotation: usize,
	                        flip: bool,
	                        mut avail: HashSet<u64>|
	 -> Result<Vec<Vec<bool>>, ()> {
		println!("------------------------------------------");
		for y in 0..size_map {
			for x in 0..size_map {
				println!("At x: {} y: {}", x, y);
				if x == 0 {
					if y == 0 {
						let id = corner_id;
						map[y][x].tile_id = id;
						map[y][x].flip = flip;
						map[y][x].rotation = initial_rotation;
						map[y][x].sides = calc_flip_rot(flip, initial_rotation, &input.tiles[&id]);
						avail.remove(&id);
						continue;
					}
					let top_tile_info = &map[y - 1][x];
					let (tile_id, pos) =
						find_tile_with_side(flip_it(top_tile_info.sides[3]), &avail)?;
					avail.remove(&tile_id);

					let rot;
					let mut flip = false;
					if pos >= 4 {
						flip = true;
					}
					match pos % 4 {
						0 => rot = 1,
						1 => rot = 0,
						2 => rot = 3,
						3 => rot = 2,
						_ => unreachable!(),
					}
					map[y][x].tile_id = tile_id;
					map[y][x].flip = flip;
					map[y][x].rotation = rot;
					map[y][x].sides = calc_flip_rot(flip, rot, &input.tiles[&tile_id]);
				} else {
					let left_tile_info = &map[y][x - 1];
					let (tile_id, pos) =
						find_tile_with_side(flip_it(left_tile_info.sides[2]), &avail)?;
					avail.remove(&tile_id);

					let rot;
					let mut flip = false;
					if pos >= 4 {
						flip = true;
					}

					match pos % 4 {
						0 => rot = 0,
						1 => rot = 3,
						2 => rot = 2,
						3 => rot = 1,
						_ => unreachable!(),
					}
					map[y][x].tile_id = tile_id;
					map[y][x].flip = flip;
					map[y][x].rotation = rot;
					map[y][x].sides = calc_flip_rot(flip, rot, &input.tiles[&tile_id]);
				}
			}
		}

		// Map of tiles created, now parsing it into a single piece
		let mut final_map: Vec<Vec<bool>> = Vec::new();
		for y in 0..(size_map * 8) {
			final_map.push(Vec::new());
		}

		for y in 0..size_map {
			for x in 0..size_map {
				let cur_data = get_tile_data(
					map[y][x].flip,
					map[y][x].rotation,
					&input.tiles[&map[y][x].tile_id],
				);

				for ty in 0..8 {
					final_map[(y * 8) + ty].extend(&cur_data[ty]);
				}
			}
		}

		Ok(final_map)
	};

	let detect_pattern = |map: &Vec<Vec<bool>>| -> Option<u64> {
		let pattern = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   \n";
		let pat: Vec<Vec<bool>> = pattern
			.lines()
			.map(|l| {
				l.chars()
					.map(|c| match c {
						'#' => true,
						_ => false,
					})
					.collect()
			})
			.collect();

		let max_search_x = map[0].len() - pat[0].len();
		let max_search_y = map.len() - pat.len();

		let mut new_map = map.clone();

		let mut found_monster = false;

		for y in 0..max_search_y {
			'thesearch: for x in 0..max_search_x {
				for py in 0..pat.len() {
					for px in 0..pat[0].len() {
						if pat[py][px] == true && new_map[y + py][x + px] != true {
							continue 'thesearch;
						}
					}
				}
				found_monster = true;
				for py in 0..pat.len() {
					for px in 0..pat[0].len() {
						if pat[py][px] == true {
							new_map[y + py][x + px] = false;
						}
					}
				}
			}
		}

		if found_monster {
			let num_true: u64 = new_map
				.iter()
				.map(|l| l.iter().map(|v| if *v { 1 } else { 0 }).sum::<u64>())
				.sum();

			Some(num_true)
		} else {
			None
		}
	};

	for c in 0..4 {
		for r in 0..4 {
			for b in [true, false].iter() {
				if let Some(res_map) = check_corner(corners[c], r, *b, available_tiles.clone()).ok()
				{
					res_map.iter().for_each(|l| {
						let line_str: String = l.iter().map(|b| if *b { '#' } else { '.' }).collect();
						println!("{}", line_str);
					});

					if let Some(res) = detect_pattern(&res_map) {
						return Ok(res);
					}
				}
			}
		}
	}

	panic!("Failed!");
}

#[test]
fn test_aoc20() {
	let mut file = File::open("input_20_test.txt").unwrap();
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file).unwrap();
	let sample_res = parse_data(&buf_file).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(20899048083289));
	assert_eq!(run_b(&sample_res).ok(), Some(273));
}
