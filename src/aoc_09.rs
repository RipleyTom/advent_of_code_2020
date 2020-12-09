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
	let mut file = File::open("input_09.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &Vec<i64>, p_length: usize) -> Result<i64, std::io::Error> {
	'main_loop: for vi in p_length..input.len() {
		for i in (vi - p_length)..vi {
			for j in (i + 1)..(i + p_length) {
				if (input[i] + input[j]) == input[vi] {
					continue 'main_loop;
				}
			}
		}
		return Ok(input[vi]);
	}

	Err(std::io::Error::new(
		std::io::ErrorKind::InvalidData,
		"All the numbers were sums?!",
	))
}

pub fn run_b(input: &Vec<i64>, to_find: i64) -> Result<i64, std::io::Error> {
	'main_loop: for i in 0..input.len() {
		let mut sum = input[i];
		let mut smallest = input[i];
		let mut largest = input[i];
		for j in (i + 1)..input.len() {
			if input[j] > largest {
				largest = input[j];
			}
			if input[j] < smallest {
				smallest = input[j];
			}

			sum += input[j];
			if sum == to_find {
				return Ok(smallest + largest);
			}
			if sum > to_find {
				continue 'main_loop;
			}
		}
	}
	Err(std::io::Error::new(
		std::io::ErrorKind::InvalidData,
		"Couldn't find continguous sum?!",
	))
}

#[test]
fn test_aoc08() {
	let sample_input =
		"35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
	let sample_res = parse_data(&sample_input).unwrap();
	let res_a = run_a(&sample_res, 5).ok();

	assert_eq!(res_a, Some(127));
	assert_eq!(run_b(&sample_res, res_a.unwrap()).ok(), Some(62));
}
