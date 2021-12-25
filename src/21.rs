use std::collections::HashMap;
use std::hash::{ Hasher, Hash };
use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Player {
	pos: u16,
	sco: u16
}

//p: Players
//t: next player to throw dices
#[derive(Debug, Clone, PartialEq, Eq)]
struct Situation {
	p: Vec::<Player>,
	t: usize
}

impl Hash for Player {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.pos.hash(state);
		self.sco.hash(state);
	}
}

impl Hash for Situation {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.p.hash(state);
		self.t.hash(state);
	}
}

impl Situation {
	fn new(pos1: u16, pos2: u16) -> Self {
		Situation {
			p: Vec::from([
				Player {
					pos: pos1,
					sco: 0
				},
				Player {
					pos: pos2,
					sco: 0
				}
			]),
			t: 0
		}
	}

	fn get_wins(self, visited: &mut HashMap<Situation, [ u64; 2 ]>) -> [ u64; 2 ] {
		if visited.contains_key(&self) {
			return *visited.get(&self).unwrap();
		}
		let mut res = [ 0, 0 ];
		for a in 1..=3 {
			for b in 1..=3 {
				for c in 1..=3 {
					let mut n = self.clone();
					n.p[n.t].pos = (n.p[n.t].pos + a + b + c - 1) % 10 + 1;
					n.p[n.t].sco += n.p[n.t].pos;
					if n.p[n.t].sco >= 21 {
						res[n.t] += 1;
					} else {
						n.t = 1 - n.t;
						let r = n.get_wins(visited);
						res[0] += r[0];
						res[1] += r[1];
					}
				}
			}
		}
		visited.insert(self, res);
		res
	}
}

fn main() {
	let p1_pos = 4;
	let p2_pos = 8;

	println!(" *: {}\n**: {}",
		{
			let mut pls = [
				Player { pos: p1_pos, sco: 0 },
				Player { pos: p2_pos, sco: 0 }
			];

			let mut dice = 1;
			let mut throwed = 0;
			let mut next_dice = || {
				let o = dice;
				dice = dice % 100 + 1;
				throwed += 1;
				o
			};
			(loop {
				let mut val = 0;
				for (i, mut p) in pls.iter_mut().enumerate() {
					p.pos += next_dice() + next_dice() + next_dice();
					p.pos = (p.pos - 1) % 10 + 1;
					p.sco += p.pos;
					if p.sco >= 1000 {
						val = pls[1 - i].sco as u32;
						break;
					}
				}
				if val > 0 {
					break val;
				}
			} * throwed)
		},
		Situation::new(p1_pos, p2_pos)
			.get_wins(&mut HashMap::new())
			.into_iter().fold(0, | a, b | cmp::max(a, b))
	);
}