use std::{ cmp, fmt };
use tools::parser;

#[derive(Copy, Clone)]
struct Cube {
	x: (i64, i64),
	y: (i64, i64),
	z: (i64, i64)
}

impl fmt::Debug for Cube {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Cube: x: {}|{}, y: {}|{}, z: {}|{}",
			self.x.0,
			self.x.1,
			self.y.0,
			self.y.1,
			self.z.0,
			self.z.1
		)
	}
}

impl Cube {
	fn new(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> Self {
		Cube { x, y, z }
	}

	fn size(&self) -> i64 {
		(self.x.1 - self.x.0)
		* (self.y.1 - self.y.0)
		* (self.z.1 - self.z.0)
	}

	fn valid(&self) -> bool {
		self.x.1 > self.x.0 && self.y.1 > self.y.0 && self.z.1 > self.z.0
	}

	fn intersects(&self, o: &Self) -> bool {
		self.x.0 < o.x.1 && self.y.0 < o.y.1 && self.z.0 < o.z.1
			&& self.x.1 > o.x.0 && self.y.1 > o.y.0 && self.z.1 > o.z.0
	}

	fn exclude(&self, o: &Self) -> Vec<Self> {
		if !self.intersects(o) {
			return vec![*self];
		}
		let get = | a: (i64, i64), b: (i64, i64), e | match e {
			0 => (a.0, cmp::max(a.0, b.0)),
			1 => (cmp::max(a.0, b.0), cmp::min(a.1, b.1)),
			2 => (cmp::min(a.1, b.1), a.1),
			_ => (0, 0)
		};
		let mut v = Vec::new();
		for x in 0..=2 {
			for y in 0..=2 {
				for z in 0..=2 {
					if x != 1 || y != 1 || z != 1 {
						v.push(Self::new(
							get(self.x, o.x, x),
							get(self.y, o.y, y),
							get(self.z, o.z, z)
						));
					}
				}
			}
		}
		v.into_iter().filter(| c | c.valid()).collect()
	}
}

#[derive(Debug)]
struct Command {
	on: bool,
	cube: Cube
}

impl Command {
	fn new(s: &str) -> Self {
		let mut sp = s.split(" ").map(| s | s.to_string());
		let on = sp.next().unwrap() == "on";
		let coord: Vec<Vec<i64>> = sp.next().unwrap().split(",").map(| s | s[2..].to_string().split("..").map(| s | s.parse::<i64>().unwrap()).collect()).collect();
		let vec_to_tuple = | v: &Vec<i64> | (v[0], v[1] + 1);
		Command {
			on,
			cube: Cube::new(
				vec_to_tuple(&coord[0]),
				vec_to_tuple(&coord[1]),
				vec_to_tuple(&coord[2])
			)
		}
	}
}

pub fn main() {
	let cmds: Vec<Command> = parser::string("22").lines().map(| s | Command::new(s)).collect();
	let mut v: Vec<Cube> = Vec::new();

	for cmd in cmds {
		let mut n = Vec::new();
		for c in v {
			n.append(&mut c.exclude(&cmd.cube));
		}
		if cmd.on {
			n.push(cmd.cube);
		}
		v = n;
	}
	println!(" *: {}\n**: {}",
		0,
		v.into_iter().map(| c | c.size()).sum::<i64>()
	);
}