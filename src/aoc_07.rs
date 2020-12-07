use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub struct BagInfo {
	can_contain: HashMap<String, usize>,
}

fn parse_data(input_data: &str) -> HashMap<String, BagInfo> {
	input_data
		.lines()
		.map(|l| {
			let b_c: Vec<&str> = l.split(" bags contain ").collect();
			let contained: HashMap<String, usize> = b_c[1]
				.split(", ")
				.filter_map(|e| {
					if e == "no other bags." {
						return None;
					}
					let bag_info: Vec<&str> = e.split(' ').collect();
					let num_bags = bag_info[0].parse().unwrap();
					let bag_name = String::from(bag_info[1]) + " " + bag_info[2];
					Some((bag_name, num_bags))
				})
				.collect();
			let bag_info = BagInfo {
				can_contain: contained,
			};

			(String::from(b_c[0]), bag_info)
		})
		.collect()
}

pub fn parse_input() -> Result<HashMap<String, BagInfo>, std::io::Error> {
	let mut file = File::open("input_07.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	Ok(parse_data(&buf_file))
}

fn bag_contain(bag_infos: &HashMap<String, BagInfo>, bag: &str, wanted: &str) -> bool {
	if bag_infos[bag].can_contain.keys().any(|v| v == wanted) {
		return true;
	}

	for k in bag_infos[bag].can_contain.keys() {
		if bag_contain(bag_infos, k, wanted) {
			return true;
		}
	}
	false
}

pub fn run_a(input: &HashMap<String, BagInfo>) -> Result<i64, std::io::Error> {
	let wanted = "shiny gold";
	let res = input
		.iter()
		.filter(|(k, _)| {
			if k == &wanted {
				return false;
			}

			bag_contain(input, k, wanted)
		})
		.count() as i64;
	Ok(res)
}

fn contains_num(bag_infos: &HashMap<String, BagInfo>, bag: &str) -> usize {
	bag_infos[bag]
		.can_contain
		.iter()
		.map(|(k, v)| (v + v * contains_num(bag_infos, k)))
		.sum()
}

pub fn run_b(input: &HashMap<String, BagInfo>) -> Result<i64, std::io::Error> {
	Ok(contains_num(input, "shiny gold") as i64)
}

#[test]
fn test_aoc07() {
	let sample_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
	let sample_res = parse_data(&sample_input);

	assert_eq!(run_a(&sample_res).ok(), Some(4));
	assert_eq!(run_b(&sample_res).ok(), Some(32));
}
