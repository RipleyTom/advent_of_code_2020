use std::cell::RefCell;
use std::rc::Rc;

type PuzzleData = Vec<u8>;

fn parse_data(input: &str) -> Result<PuzzleData, std::io::Error> {
	let res = input
		.chars()
		.map(|c| c.to_digit(10).unwrap() as u8)
		.collect();
	Ok(res)
}

pub fn parse_input() -> Result<PuzzleData, std::io::Error> {
	parse_data("538914762")
}

#[derive(Debug)]
struct Node {
	data: u32,
	next: Option<Rc<RefCell<Node>>>,
}

impl Node {
	fn print(&self) -> String {
		let mut cur_node = self.next.clone().unwrap();
		let mut res_string = String::new();
		for _ in 0..8 {
			res_string += &cur_node.borrow().data.to_string();
			let nn = cur_node.borrow().next.clone().unwrap();
			cur_node = nn;
		}
		res_string
	}
}

pub fn run_a(input: &PuzzleData) -> Result<String, std::io::Error> {
	let init_node = Rc::new(RefCell::new(Node {
		data: input[0] as u32,
		next: None,
	}));

	let mut cur_node = init_node.clone();
	let mut one_node = None;
	for i in 1..9 {
		let new_node = Rc::new(RefCell::new(Node {
			data: input[i] as u32,
			next: None,
		}));

		if input[i] == 1 {
			one_node = Some(new_node.clone());
		}

		cur_node.borrow_mut().next = Some(new_node.clone());
		cur_node = new_node;
	}
	cur_node.borrow_mut().next = Some(init_node.clone());

	let get_prev = |v: u32| -> u32 {
		match v {
			1 => 9,
			_ => (v - 1),
		}
	};

	let get_next_3 = |cur_node: Rc<RefCell<Node>>| -> Rc<RefCell<Node>> {
		let init_node = cur_node.clone();
		let first_of_3 = cur_node.borrow().next.clone().unwrap();
		let mut cur_node = init_node.clone();
		for _ in 0..3 {
			let nn = cur_node.borrow().next.clone().unwrap();
			cur_node = nn;
		}
		let next_node = cur_node.borrow().next.clone();
		init_node.borrow_mut().next = next_node.clone();

		first_of_3
	};

	let insert_3 = |wti: Rc<RefCell<Node>>, to_insert: Rc<RefCell<Node>>| {
		let initial_next = wti.borrow().next.clone();

		let mut cur_node = to_insert.clone();
		for _ in 0..2 {
			let nn = cur_node.borrow().next.clone().unwrap();
			cur_node = nn;
		}
		// Edit the 3 nodes
		cur_node.borrow_mut().next = initial_next.clone();

		// Edit insert node
		wti.borrow_mut().next = Some(to_insert);
	};

	let mut cur_node = init_node.clone();

	for _ in 0..100 {
		let cur_cup = cur_node.borrow().data;
		let next_3 = get_next_3(cur_node.clone());
		let mut look_cup = get_prev(cur_cup);
		'loop_find: loop {
			let mut node3_index = next_3.clone();
			for _ in 0..3 {
				if node3_index.borrow().data == look_cup {
					look_cup = get_prev(look_cup);
					continue 'loop_find;
				}
				let nn = node3_index.borrow().next.clone().unwrap();
				node3_index = nn;
			}
			break;
		}

		let mut cur_nfind = cur_node.borrow().next.clone().unwrap();
		'loop_find2: loop {
			if cur_nfind.borrow().data == look_cup {
				break 'loop_find2;
			}
			let nn = cur_nfind.borrow().next.clone().unwrap();
			cur_nfind = nn;
		}

		insert_3(cur_nfind, next_3);

		let nn = cur_node.borrow().next.clone().unwrap();
		cur_node = nn;
	}

	Ok(one_node.unwrap().borrow().print())
}

pub fn run_a_impl_2(input: &PuzzleData) -> Result<String, std::io::Error> {
	let mut next_array: Box<[u32; 10]> = Box::new([0; 10]);
	for i in 0..8 {
		next_array[input[i] as usize] = input[i+1] as u32;
	}
	next_array[input[8] as usize] = input[0] as u32;

	let get_prev = |v: u32| -> u32 {
		match v {
			1 => 9,
			_ => (v - 1),
		}
	};

	let mut cur_cup = input[0] as u32;
	for r in 0..100 {
		let first_of_3 = next_array[cur_cup as usize];
		let mut values = [0; 3];
		values[0] = first_of_3;
		values[1] = next_array[values[0] as usize];
		values[2] = next_array[values[1] as usize];

		let mut look_cup = get_prev(cur_cup);
		while values.contains(&look_cup) {
			look_cup = get_prev(look_cup);
		}

		next_array[cur_cup as usize] = next_array[values[2] as usize];

		let old_next = next_array[look_cup as usize];
		next_array[look_cup as usize] = values[0];
		next_array[values[2] as usize] = old_next;
		cur_cup = next_array[cur_cup as usize];
	}

	let mut res_string = String::new();
	cur_cup = 1;
	for i in 0..8 {
		let next = next_array[cur_cup as usize];
		res_string += &next.to_string();
		cur_cup = next;
	}

	Ok(res_string)
}

pub fn run_b(input: &PuzzleData) -> Result<u64, std::io::Error> {
	let mut next_array: Vec<u32> = Vec::new();
	next_array.resize(1_000_001, 0);
	for i in 0..8 {
		next_array[input[i] as usize] = input[i+1] as u32;
	}
	next_array[input[8] as usize] = 10;

	for i in 10..1_000_000 {
		next_array[i] = (i + 1) as u32;
	}
	next_array[1_000_000] = input[0] as u32;

	let get_prev = |v: u32| -> u32 {
		match v {
			1 => 1_000_000,
			_ => (v - 1),
		}
	};

	let mut cur_cup = input[0] as u32;
	for _ in 0..10_000_000 {
		let first_of_3 = next_array[cur_cup as usize];
		let mut values = [0; 3];
		values[0] = first_of_3;
		values[1] = next_array[values[0] as usize];
		values[2] = next_array[values[1] as usize];

		let mut look_cup = get_prev(cur_cup);
		while values.contains(&look_cup) {
			look_cup = get_prev(look_cup);
		}

		next_array[cur_cup as usize] = next_array[values[2] as usize];

		let old_next = next_array[look_cup as usize];
		next_array[look_cup as usize] = values[0];
		next_array[values[2] as usize] = old_next;
		cur_cup = next_array[cur_cup as usize];
	}

	let next_1 = next_array[1];
	let next_next_1 = next_array[next_1 as usize];

	Ok(next_1 as u64 * next_next_1 as u64)
}

#[test]
fn test_aoc23() {
	let sample_res = parse_data(&"389125467").unwrap();

	assert_eq!(run_a(&sample_res).ok(), Some("67384529".to_string()));
	assert_eq!(run_a_impl_2(&sample_res).ok(), Some("67384529".to_string()));
	assert_eq!(run_b(&sample_res).ok(), Some(149245887792));
}
