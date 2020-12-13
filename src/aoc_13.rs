use std::fs::File;
use std::io::Read;

pub struct LineInfo {
	earliest: i64,
	bus_ids: Vec<i64>,
}

fn parse_data(input_data: &str) -> Result<LineInfo, std::io::Error> {
	let data: Vec<&str> = input_data.lines().collect();
	let earliest = data[0].parse().unwrap();
	let bus_ids = data[1]
		.split(',')
		.map(|v| if v == "x" { 0 } else { v.parse().unwrap() })
		.collect();

	Ok(LineInfo { earliest, bus_ids })
}

pub fn parse_input() -> Result<LineInfo, std::io::Error> {
	let mut file = File::open("input_13.txt")?;
	let mut buf_file = String::new();
	file.read_to_string(&mut buf_file)?;

	parse_data(&buf_file)
}

pub fn run_a(input: &LineInfo) -> Result<i64, std::io::Error> {
	let mut best_id = 0;
	let mut cur_min = i64::MAX;
	for bus_id in input.bus_ids.iter() {
		if *bus_id == 0 {
			continue;
		}
		let time_since_left = input.earliest % bus_id;
		let time_since_here_again = (bus_id - time_since_left) % bus_id;
		if time_since_here_again < cur_min {
			cur_min = time_since_here_again;
			best_id = *bus_id;
		}
	}

	Ok(best_id * cur_min)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}
 
pub fn run_b(input: &LineInfo) -> Result<i64, std::io::Error> {
	let mut modulii: Vec<i64> = Vec::new();
	let mut residues: Vec<i64> = Vec::new();

	for t in 0..input.bus_ids.len() {
		let id = input.bus_ids[t];
		if id == 0 {
			continue;
		}

		modulii.push(id);
		residues.push((id - t as i64)%id);
	}

	Ok(chinese_remainder(&residues, &modulii).unwrap())
}

#[test]
fn test_aoc13() {
	let sample_input = "939\n7,13,x,x,59,x,31,19";
	let sample_res = parse_data(&sample_input).unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some(295));
	assert_eq!(run_b(&sample_res).ok(), Some(1068781));
}
