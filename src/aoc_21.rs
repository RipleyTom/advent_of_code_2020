use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

type PuzzleData = Vec<(HashSet<String>, HashSet<String>)>;

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	Ok(input.lines().map(|l| {
		let ingr_aler: Vec<&str> = l.split(" (contains ").collect();
		let ingredients: HashSet<String> = ingr_aler[0].split(' ').map(|i| String::from(i)).collect();
		let aler_str = &ingr_aler[1][0..ingr_aler[1].len() - 1];
		let alergenes: HashSet<String> = aler_str.split(", ").map(|a| String::from(a)).collect();
		(ingredients, alergenes)
	}).collect())
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	let mut file = File::open("input_21.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut all_ingr: HashMap<String, u64> = HashMap::new();
	let mut aler_ingr: HashMap<String, HashSet<String>> = HashMap::new();

	for (ingr, aler) in input {
		ingr.iter().for_each(|i| { 
			let ingr_ent = all_ingr.entry(i.clone()).or_insert(0);
			*ingr_ent += 1;
		});

		for a in aler {
			let cur_aler = aler_ingr.entry(a.clone()).or_insert(ingr.clone());
			let diff = cur_aler.intersection(ingr);
			*cur_aler = diff.map(|i| i.clone()).collect();
		}
	}

	for (a, li) in &aler_ingr {
		println!("Possibilities for {} are {}({:?})", a, li.len(), li);
		for ci in li {
			all_ingr.remove(ci);
		}
	}

	let sum = all_ingr.values().sum();

	Ok(sum)
}

fn recurse_cleanup(data: &mut Vec<(String, HashSet<String>)>) {
	let mut modified = false;
	let len = data.len();
	for i in 0..len {
		if data[i].1.len() == 1 {
			for si in 0..len {
				if i == si {
					continue;
				}
				let val = data[i].1.iter().next().unwrap().clone();
				modified |= data[si].1.remove(&val);
			}

			if modified {
				return recurse_cleanup(data);
			}
		}
	}
}

pub fn run_b(input: &PuzzleData) -> Result<String, std::io::Error> {
	let mut all_ingr: HashMap<String, u64> = HashMap::new();
	let mut aler_ingr: HashMap<String, HashSet<String>> = HashMap::new();

	for (ingr, aler) in input {
		ingr.iter().for_each(|i| { 
			let ingr_ent = all_ingr.entry(i.clone()).or_insert(0);
			*ingr_ent += 1;
		});

		for a in aler {
			let cur_aler = aler_ingr.entry(a.clone()).or_insert(ingr.clone());
			let diff = cur_aler.intersection(ingr);
			*cur_aler = diff.map(|i| i.clone()).collect();
		}
	}

	for (a, li) in &aler_ingr {
		println!("Possibilities for {} are {}({:?})", a, li.len(), li);
		for ci in li {
			all_ingr.remove(ci);
		}
	}

	let mut pre_sorted: Vec<(String, HashSet<String>)> = aler_ingr.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
	recurse_cleanup(&mut pre_sorted);

	let mut sorted_aler_ingr: Vec<(String, Vec<String>)> = pre_sorted.iter().map(|(k, v)| (k.clone(), v.iter().map(|vs| vs.clone()).collect())).collect();
	for (_, vs) in &mut sorted_aler_ingr {
		vs.sort();
	}
	sorted_aler_ingr.sort();

	let mut final_str = String::new();
	for (_, ingr) in sorted_aler_ingr {
		for ing in ingr {
			final_str += &ing;
			final_str += ",";
		}
	}

	let len = final_str.len();
	let final_str = String::from(&final_str[0..(len - 1)]);


	Ok(final_str)
}

#[test]
fn test_aoc21() {
	let sample_res = parse_data(&"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)").unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(5));
	assert_eq!(run_b(&sample_res).ok(), Some(String::from("mxmxvkd,sqjhc,fvjkl")));
}
