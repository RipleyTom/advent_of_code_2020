use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

struct Descriptor {
	name: String,
	min: [u64; 2],
	max: [u64; 2],
}

pub struct PuzzleData {
	descriptors: Vec<Descriptor>,
	ticket: Vec<u64>,
	nearby_tickets: Vec<Vec<u64>>,
}

fn parse_data(input_data: &str) -> Result<PuzzleData, std::io::Error> {
	let mut descriptors: Vec<Descriptor> = Vec::new();
	let mut ticket: Vec<u64> = Vec::new();
	let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();

	enum ParsingState {
		Descriptors,
		Ticket,
		NearbyTickets,
	};

	let mut state = ParsingState::Descriptors;

	for l in input_data.lines() {
		if l.is_empty() {
			continue;
		}
		if l == "your ticket:" {
			state = ParsingState::Ticket;
			continue;
		}
		if l == "nearby tickets:" {
			state = ParsingState::NearbyTickets;
			continue;
		}

		match state {
			ParsingState::Descriptors => {
				let name_data: Vec<&str> = l.split(": ").collect();
				let data: Vec<&str> = name_data[1].split(" or ").collect();
				let mut min = [0; 2];
				let mut max = [0; 2];
				for i in 0..2 {
					let min_max: Vec<&str> = data[i].split('-').collect();
					min[i] = min_max[0].parse().unwrap();
					max[i] = min_max[1].parse().unwrap();
				}

				descriptors.push(Descriptor {
					name: String::from(name_data[0]),
					min,
					max,
				});
			}
			ParsingState::Ticket => {
				ticket = l.split(',').map(|v| v.parse().unwrap()).collect();
			}
			ParsingState::NearbyTickets => {
				let new_ticket: Vec<u64> = l.split(',').map(|v| v.parse().unwrap()).collect();
				nearby_tickets.push(new_ticket);
			}
		}
	}

	Ok(PuzzleData {
		descriptors,
		ticket,
		nearby_tickets,
	})
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_16.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut error_rate = 0;

	for t in &input.nearby_tickets {
		'value_loop: for v in t {
			for d in &input.descriptors {
				if (*v >= d.min[0] && *v <= d.max[0]) || (*v >= d.min[1] && *v <= d.max[1]) {
					continue 'value_loop;
				}
			}
			error_rate += v;
		}
	}

	Ok(error_rate)
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut valid_tickets: Vec<Vec<u64>> = Vec::new();

	'ticket_loop: for t in &input.nearby_tickets {
		'value_loop: for v in t {
			for d in &input.descriptors {
				if (*v >= d.min[0] && *v <= d.max[0]) || (*v >= d.min[1] && *v <= d.max[1]) {
					continue 'value_loop;
				}
			}
			continue 'ticket_loop;
		}
		valid_tickets.push(t.clone());
	}

	let mut cannot_be: Vec<HashSet<u64>> = Vec::new();
	for _ in 0..input.descriptors.len() {
		cannot_be.push(HashSet::new());
	}

	for t in &valid_tickets {
		let mut vi = 0;
		for v in t {
			let mut di = 0;
			for d in &input.descriptors {
				if (*v < d.min[0] || *v > d.max[0]) && (*v < d.min[1] || *v > d.max[1]) {
					cannot_be[di].insert(vi);
				}
				di += 1;
			}
			vi += 1;
		}
	}

	let mut full_set: HashSet<u64> = HashSet::new();
	for i in 0..input.ticket.len() {
		full_set.insert(i as u64);
	}

	let mut possible: Vec<HashSet<u64>> = cannot_be
		.iter()
		.map(|p| full_set.difference(p).map(|v| *v).collect())
		.collect();

	let mut changed = true;
	while changed {
		changed = false;
		for i in 0..possible.len() {
			if possible[i].len() == 1 {
				let to_remove = *possible[i].iter().next().unwrap();
				for si in 0..possible.len() {
					if i == si {
						continue;
					}
					changed |= possible[si].remove(&to_remove);
				}
			}
		}
	}

	let mut min_poss = 0;
	for i in 1..possible.len() {
		if possible[i].len() < possible[min_poss].len() {
			min_poss = i;
		}
	}

	let mut result = 1;
	for i in 0..possible.len() {
		if input.descriptors[i].name.contains("departure") {
			result *= input.ticket[*possible[i].iter().next().unwrap() as usize];
		}
	}

	Ok(result)
}

#[test]
fn test_aoc16() {
	let sample_input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
	let sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(71));
}
