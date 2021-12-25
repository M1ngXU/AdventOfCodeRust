use std::collections::{ BinaryHeap, HashSet };
use std::{ fmt, cmp, cmp::Ordering };

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Location {
	None,
	Amber,
	Bronze,
	Copper,
	Desert
}

impl Location {
	fn get_symbol(&self) -> String {
		(match self {
			Location::Amber => "A",
			Location::Bronze => "B",
			Location::Copper => "C",
			Location::Desert => "D",
			Location::None => "."
		}).to_string()
	}

	fn get_energy(&self) -> usize {
		match self {
			Location::Amber => 1,
			Location::Bronze => 10,
			Location::Copper => 100,
			Location::Desert => 1000,
			Location::None => 0
		}
	}

	fn get_side_door_index(&self) -> usize {
		match self {
			Location::Amber => 0,
			Location::Bronze => 1,
			Location::Copper => 2,
			Location::Desert => 3,
			Location::None => panic!("Tried to get invalid room_side_index.")
		}
	}
	
	fn by_index(i: usize) -> Self {
		match i {
			0 => Location::Amber,
			1 => Location::Bronze,
			2 => Location::Copper,
			3 => Location::Desert,
			_ => panic!("Tried to get invalid location (index: {}).", i)
		}
	}
}

#[derive(Copy, Clone)]
struct Situation {
	hallway: [ Location; 11 ],
	side_rooms: [ [ Location; 4 ]; 4 ],
	energy: usize
}

impl fmt::Display for Situation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}
impl Ord for Situation {
    fn cmp(&self, other: &Self) -> Ordering {
      other.energy.cmp(&self.energy)
    }
}
impl PartialOrd for Situation {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(other.energy.cmp(&self.energy))
  }
}
impl PartialEq for Situation {
  fn eq(&self, other: &Self) -> bool {
    self.energy == other.energy
  }
}
impl Eq for Situation {}

impl Situation {
	fn new(side_rooms: [ [ Location; 4 ]; 4 ]) -> Self {
		Situation {
			hallway: [
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None,
				Location::None
			],
			side_rooms,
			energy: 0
		}
	}

	fn finished(&self) -> bool {
		!self.side_rooms.iter().enumerate().any(| (i, s) | s.iter().any(| l | *l != Location::by_index(i)))
	}

	fn to_string(&self) -> String {
		format!("#############\n#{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#  \n  #{}#{}#{}#{}#  \n  #{}#{}#{}#{}#  \n  #########  ",
			self.hallway.map(| l | l.get_symbol()).join(""),
			self.side_rooms[0][0].get_symbol(),
			self.side_rooms[1][0].get_symbol(),
			self.side_rooms[2][0].get_symbol(),
			self.side_rooms[3][0].get_symbol(),
			self.side_rooms[0][1].get_symbol(),
			self.side_rooms[1][1].get_symbol(),
			self.side_rooms[2][1].get_symbol(),
			self.side_rooms[3][1].get_symbol(),
			self.side_rooms[0][2].get_symbol(),
			self.side_rooms[1][2].get_symbol(),
			self.side_rooms[2][2].get_symbol(),
			self.side_rooms[3][2].get_symbol(),
			self.side_rooms[0][3].get_symbol(),
			self.side_rooms[1][3].get_symbol(),
			self.side_rooms[2][3].get_symbol(),
			self.side_rooms[3][3].get_symbol()
		)
	}
}

fn main() {
	let s = Situation::new([
		[
			Location::Amber,
			Location::Desert,
			Location::Desert,
			Location::Desert
		], [
			Location::Copper,
			Location::Copper,
			Location::Bronze,
			Location::Desert
		], [
			Location::Bronze,
			Location::Bronze,
			Location::Amber,
			Location::Amber
		], [
			Location::Bronze,
			Location::Amber,
			Location::Copper,
			Location::Copper
		]
		// for part one the last 2 amphipod have to be AA/BB/CC/DD (they won't be moved)
		/*[
			Location::Amber,
			Location::Desert,
			Location::Amber,
			Location::Amber
		], [
			Location::Copper,
			Location::Desert,
			Location::Bronze,
			Location::Bronze
		], [
			Location::Bronze,
			Location::Amber,
			Location::Copper,
			Location::Copper
		], [
			Location::Bronze,
			Location::Copper,
			Location::Desert,
			Location::Desert
		]*/
	]);
	let possible_end_pos = [ 0, 1, 3, 5, 7, 9, 10 ];
	let mut visited = HashSet::new();
	visited.insert(s.to_string());
	let mut stack = BinaryHeap::new();
	stack.push(s);
	let mut seen = 0;
	while let Some(current) = stack.pop() {
		if seen % 1000 == 0 {
			println!("Current energy: {}; Seen {}; Stack has now a size of {}", current.energy, seen, stack.len());
		}
		seen += 1;
		if current.finished() {
			println!("Energy: {}\n{}", current.energy, current);
			break;
		}

		let hallway_empty = | mut from: usize, mut to: usize, o | {
			if from > to {
				let t = to;
				to = from - o;
				from = t;
			} else {
				from += o;
			}
			!(from..=to).into_iter().any(| i | current.hallway[i] != Location::None)
		};

		let mut moved_into_side = false;

		for (i, a) in current.hallway.iter().enumerate().filter(| (_, a) | *(*a) != Location::None) {
			let side_door_index = a.get_side_door_index();
			if hallway_empty(i, side_door_index * 2 + 2, 1) && !current.side_rooms[side_door_index].iter().any(| l | *l != Location::by_index(side_door_index) && *l != Location::None) {
				if let Some(r_i) = current.side_rooms[side_door_index].iter().rposition(| l | *l == Location::None) {
					let to_be_moved = current.hallway[i];
					let mut new_situation = current.clone();
					new_situation.side_rooms[side_door_index][r_i] = to_be_moved;
					new_situation.hallway[i] = Location::None;
					new_situation.energy += to_be_moved.get_energy() * (cmp::max(i, side_door_index * 2 + 2) - cmp::min(i, side_door_index * 2 + 2) + r_i + 1);
					if !visited.contains(&new_situation.to_string()) {
						visited.insert(new_situation.to_string());
						stack.push(new_situation);
					}
					moved_into_side = true;
				}
			}
		}

		if moved_into_side {
			continue;
		}

		for (sr_i, sr) in current.side_rooms
				.iter().enumerate()
				.filter(| (i, sr) | sr.iter().any(| l | *l != Location::by_index(*i) && *l != Location::None)) {
			if let Some(r_i) = sr.iter().position(| a | *a != Location::None) {
				let hallway_index = sr_i * 2 + 2;
				for i in possible_end_pos {
					if hallway_empty(i, hallway_index, 0) {
						let to_be_moved = sr[r_i];
						let mut new_situation = current.clone();
						new_situation.side_rooms[sr_i][r_i] = Location::None;
						new_situation.hallway[i] = to_be_moved;
						new_situation.energy += to_be_moved.get_energy() * (cmp::max(i, hallway_index) - cmp::min(i, hallway_index) + r_i + 1);
						if !visited.contains(&new_situation.to_string()) {
							visited.insert(new_situation.to_string());
							stack.push(new_situation);
						}
					}
				}
			}
		}
	}
}