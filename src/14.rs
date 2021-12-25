use tools::parser;
use std::collections::HashMap;
use std::hash::{ Hasher, Hash };

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pair(char, char);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TemplatePair {
	pair: Pair,
	insertion: char,
	amount: u64
}

impl Hash for TemplatePair {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.pair.hash(state);
	}
}

impl TemplatePair {
	fn new(rule: &str) -> Self {
		let r: Vec<String> = rule.split(" -> ").map(|s| s.to_string()).collect();
		let mut iter = r[0].chars();
		let p1 = iter.next().unwrap();
		let p2 = iter.next().unwrap();
		Self { pair: Pair(p1, p2), insertion: r[1].chars().next().unwrap(), amount: 0 }
	}

	fn new_by_pair(c1: char, c2: char, amount: u64, v: Vec<Self>) -> Self {
		Self {
			pair: Pair (c1, c2),
			insertion: Self::get(c1, c2, v).insertion,
			amount
		}
	}

	fn get(c1: char, c2: char, v: Vec<Self>) -> Self {
		for p in v {
			if p.pair == Pair(c1, c2) {
				return p;
			}
		}
		panic!("Couldn't find {}|{}!", c1, c2);
	}

	fn get_mut(c1: char, c2: char, v: &mut Vec<Self>) -> &mut Self {
		for p in v.iter_mut() {
			if p.pair == Pair(c1, c2) {
				return p;
			}
		}
		panic!("Couldn't find {}|{}!", c1, c2);
	}

	fn get_by_pair(p: Pair, v: &mut Vec<Self>) -> &mut Self {
		Self::get_mut(p.0, p.1, v)
	}

	fn insert(v: &mut Vec<Self>) {
		let mut np = HashMap::new();
		for p in v.iter() {
			let mut add = | c1, c2 | {
				let n = Self::new_by_pair(c1, c2, 0, v.to_vec());
				if np.contains_key(&n) {
					*np.get_mut(&n).unwrap() += p.amount;
				} else {
					np.insert(n, p.amount);
				}
			};
			add(p.pair.0, p.insertion);
			add(p.insertion, p.pair.1);
		}
		for p in v.iter_mut() {
			p.amount = 0;
		}
		for (k, a) in np.into_iter() {
			Self::get_by_pair(k.pair, v).amount = a;
		}
	}
}

fn calc_res(template_pairs: &Vec<TemplatePair>) -> u64 {
	let mut chars = HashMap::new();
	let mut add = | c, a | {
		if !chars.contains_key(&c) {
			chars.insert(c, 0);
		}
		*chars.get_mut(&c).unwrap() += a;
	};

	for tp in template_pairs {
		add(tp.pair.0, tp.amount);
		add(tp.pair.1, tp.amount);
	}
	let mut v = chars.into_iter().map(| (_, a) | (a + 1) / 2).collect::<Vec<u64>>();
	v.sort();
	v[v.len() - 1] - v[0]
}

fn main() {
	let mut template_pairs = Vec::new();

	let input = parser::string("14").split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
	for s in input[1].lines() {
		template_pairs.push(TemplatePair::new(s));
	}

	let start_template = input[0].chars().collect::<Vec<char>>();
	for (i, p) in start_template.iter().enumerate() {
		if i > 0 {
			TemplatePair::get_mut(
				start_template[i - 1],
				*p,
				&mut template_pairs
			).amount += 1;
		}
	}
	let mut p1 = 0;
	for i in 0..40 {
		if i == 10 {
			p1 = calc_res(&template_pairs);
		}
		TemplatePair::insert(&mut template_pairs);
	}
	println!(" *: {}\n**: {}", p1, calc_res(&template_pairs));
}