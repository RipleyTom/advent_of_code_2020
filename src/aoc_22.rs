use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

type PuzzleData = (VecDeque<u64>, VecDeque<u64>);

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	let mut p1_cards: VecDeque<u64> = VecDeque::new();
	let mut p2_cards: VecDeque<u64> = VecDeque::new();
	let mut cur_cards = 1;

	input.lines().for_each(|l| match l {
		"Player 1:" => cur_cards = 1,
		"Player 2:" => cur_cards = 2,
		"" => {}
		_ => {
			if cur_cards == 1 {
				p1_cards.push_back(l.parse().unwrap())
			} else {
				p2_cards.push_back(l.parse().unwrap())
			}
		}
	});

	Ok((p1_cards, p2_cards))
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_22.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let (mut p1d, mut p2d) = input.clone();

	while !p1d.is_empty() && !p2d.is_empty() {
		let c1 = p1d.pop_front().unwrap();
		let c2 = p2d.pop_front().unwrap();

		if c1 > c2 {
			p1d.push_back(c1);
			p1d.push_back(c2);
		} else {
			p2d.push_back(c2);
			p2d.push_back(c1);
		}
	}

	let winner = if p1d.is_empty() { &p2d } else { &p1d };

	let sum: u64 = winner
		.iter()
		.rev()
		.enumerate()
		.map(|(pos, v)| ((pos + 1) as u64) * v)
		.sum();

	Ok(sum)
}

enum Players {
	Player1,
	Player2,
}

fn recurse_game(mut p1d: VecDeque<u64>, mut p2d: VecDeque<u64>) -> (VecDeque<u64>, VecDeque<u64>) {
	let mut already_seen: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::new();

	while !p1d.is_empty() && !p2d.is_empty() {

		if already_seen.contains(&(p1d.clone(), p2d.clone())) {
			println!("Win for Player 1 through repetition!");
			p2d.clear();
			return (p1d, p2d);
		}
		already_seen.insert((p1d.clone(), p2d.clone()));

		let c1 = p1d.pop_front().unwrap();
		let c2 = p2d.pop_front().unwrap();

		println!("C1: {} C2: {}", c1, c2);

		if p1d.len() >= c1 as usize && p2d.len() >= c2 as usize {
			println!("Engaging in recursive game!");
			let mut sub_p1d = VecDeque::new();
			for i in 0..c1 {
				sub_p1d.push_back(p1d[i as usize]);
			}
			let mut sub_p2d = VecDeque::new();
			for i in 0..c2 {
				sub_p2d.push_back(p2d[i as usize]);
			}

			println!("Player1: {:?}, Player2: {:?}", sub_p1d, sub_p2d);
			let (_, sub_2) = recurse_game(sub_p1d, sub_p2d);

			if sub_2.is_empty() {
				println!("Player 1 won sub game!");
				p1d.push_back(c1);
				p1d.push_back(c2);
			} else {
				println!("Player 2 won sub game!");
				p2d.push_back(c2);
				p2d.push_back(c1);
			}
		} else {
			if c1 > c2 {
				println!("Player 1 won through > card!");
				p1d.push_back(c1);
				p1d.push_back(c2);
			} else {
				println!("Player 2 won through > card!");
				p2d.push_back(c2);
				p2d.push_back(c1);
			}
		}
	}

	(p1d, p2d)
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let (p1d, p2d) = recurse_game(input.0.clone(), input.1.clone());

	let winner = if p1d.is_empty() { &p2d } else { &p1d };

	let sum: u64 = winner
		.iter()
		.rev()
		.enumerate()
		.map(|(pos, v)| ((pos + 1) as u64) * v)
		.sum();

	Ok(sum)
}

#[test]
fn test_aoc22() {
	let sample_res = parse_data(&"Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10").unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(306));
	assert_eq!(run_b(&sample_res).ok(), Some(291));
}
