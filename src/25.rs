use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
	Right,
	Down,
	None
}
impl Direction {
	fn new(c: char) -> Self {
		match c {
			'>' => Direction::Right,
			'v' => Direction::Down,
			'.' => Direction::None,
			_ => panic!("Unknown Direction {}!", c)
		}
	}

	fn to_str(&self) -> &str {
		match self {
			Direction::Right => ">",
			Direction::Down => "v",
			Direction::None => "."
		}
	}
}
struct Grid {
	grid: HashMap<usize, HashMap<usize, Direction>>,
	width: usize,
	height: usize
}

impl Debug for Grid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl Grid {
	fn new(width: usize, height: usize) -> Self {
		Grid {
			grid: HashMap::new(),
			width,
			height
		}
	}

	fn to_string(&self) -> String {
		let mut s = String::new();
		for y in 0..self.height {
			for x in 0..self.width {
				s += &self.get(x, y).to_str();
			}
			s += "\n";
		}
		s
	}

	fn check_x_y(&self, x: usize, y: usize) {
		if x >= self.width {
			panic!("Width is {}, but tried to access {}.", self.width, x);
		}
		if y >= self.height {
			panic!("Height is {}, but tried to access {}.", self.height, y);
		}
	}

	fn get(&self, x: usize, y: usize) -> Direction {
		self.check_x_y(x, y);
		if let Some(gy) = self.grid.get(&y) {
			if let Some(gx) = gy.get(&x) {
				return *gx;
			}
		}
		Direction::None
	}

	fn get_mut(&mut self, x: usize, y: usize) -> &mut Direction {
		self.grid.entry(y).or_insert(HashMap::new()).entry(x).or_insert(Direction::None)
	}

	fn set(&mut self, x: usize, y: usize, d: Direction) {
		*self.get_mut(x, y) = d;
	}
}

fn main() {
	let input = tools::parser::string("25");

	let width = input.split("\n").next().unwrap().len();
	let height = input.len() / width;
	
	let mut grid = Grid::new(width, height);
	for (y, s) in input.split("\n").enumerate() {
		for (x, d) in s.chars().map(| c | Direction::new(c)).enumerate() {
			grid.set(x, y, d);
		}
	}

	let mut tick = 0;
	let mut moved = true;

	while moved == true {
		moved = false;
		let mut new_grid = Grid::new(width, height);
		
		for y in 0..height {
			for x in 0..width {
				if grid.get(x, y) == Direction::Right {
					let new_x = (x + 1) % width;
					if grid.get(new_x, y) == Direction::None {
						*new_grid.get_mut(new_x, y) = Direction::Right;
						moved = true;
					} else {
						*new_grid.get_mut(x, y) = Direction::Right;
					}
				}
			}
		}
		for y in 0..height {
			for x in 0..width {
				if grid.get(x, y) == Direction::Down {
					let new_y = (y + 1) % height;
					if new_grid.get(x, new_y) == Direction::None && grid.get(x, new_y) != Direction::Down {
						*new_grid.get_mut(x, new_y) = Direction::Down;
						moved = true;
					} else {
						*new_grid.get_mut(x, y) = Direction::Down;
					}
				}
			}
		}
		grid = new_grid;
		tick += 1;
	}

	println!("{}", tick);
}