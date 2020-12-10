use std::fs::File;
use std::io::Read;

fn parse_data(input_data: &str) -> Result<Vec<i64>, std::io::Error> {
	input_data
		.lines()
		.map(|l| l.parse())
		.collect::<Result<Vec<i64>, _>>()
		.map_err(|_| {
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"There is a non-number in the list!",
			)
		})
}

pub fn parse_input() -> Result<Vec<i64>, std::io::Error> {
	let mut file = File::open("input_10.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &Vec<i64>) -> Result<i64, std::io::Error> {
	let mut cur_jolt = 0;
	let mut num_1_jolt = 0;
	let mut num_3_jolt = 1;

	let mut sorted_input: Vec<i64> = input.clone();
	sorted_input.sort();

	for v in sorted_input.iter() {
		match v - cur_jolt {
			1 => num_1_jolt += 1,
			2 => {}
			3 => num_3_jolt += 1,
			_ => {
				return Err(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					"Adapter wasn't in range!",
				));
			}
		}
		cur_jolt = *v;
	}

	Ok(num_1_jolt * num_3_jolt)
}

fn get_num_arr(input: Vec<i64>, index: usize) -> i64 {
	let mut arr = 1;
	for i in index..input.len() - 1 {
		if input[i + 1] - input[i - 1] <= 3 {
			let mut new_vec: Vec<i64> = input.clone();
			new_vec.remove(i);
			arr += get_num_arr(new_vec, i);
		}
	}

	arr
}

pub fn run_b(input: &Vec<i64>) -> Result<i64, std::io::Error> {
	let mut num_arr = 1;

	let mut sorted_input: Vec<i64> = input.clone();
	sorted_input.sort();
	let mut input: Vec<i64> = vec![0];
	input.extend(sorted_input);
	input.push(input[input.len() - 1] + 3);
	let input = input;

	let mut cur_series: Vec<i64> = Vec::new();

	for i in 0..(input.len() - 1) {
		cur_series.push(input[i]);
		if input[i + 1] - input[i] == 3 {
			num_arr *= get_num_arr(cur_series.clone(), 1);
			cur_series.clear();
		}
	}

	Ok(num_arr)
}

#[test]
fn test_aoc10() {
	let sample_input_1 = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
	let sample_input_2 = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n";
	let sample_res_1 = parse_data(&sample_input_1).unwrap();
	let sample_res_2 = parse_data(&sample_input_2).unwrap();

	assert_eq!(run_a(&sample_res_1).ok(), Some(35));
	assert_eq!(run_a(&sample_res_2).ok(), Some(220));
	assert_eq!(run_b(&sample_res_1).ok(), Some(8));
	assert_eq!(run_b(&sample_res_2).ok(), Some(19208));
}
