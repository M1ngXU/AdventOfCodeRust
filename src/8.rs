use tools::parser;

fn main() {
	let mut input = parser::number_array("8", ",");
	input.sort();
	let min = | a: i32, b: i32 | if a > b { b } else { a };
	let dist = | a: i32, b: i32 | if a > b { a - b } else { b - a };
	let calc_total_fuel_cost = | med, cost: &dyn Fn(i32) -> i32 | {
		let mut c = 0;
		for pos in input.iter() {
			c += &cost(dist(*pos, med));
		}
		c
	};
	println!(" *: {}\n**: {}", 
		calc_total_fuel_cost(
			match input.len() - 1 {
				n if n % 2 == 1 => (input[n / 2] + input[n / 2 + 1]) / 2,
				n => input[n / 2]
			}, &| d: i32 | d
		),
		{
			let gauss = | d: i32 | d * (d + 1) / 2;
			let calc_cost = | avg | calc_total_fuel_cost(avg, &gauss);
			let avg = input.iter().sum::<i32>() as f64 / input.len() as f64;
			min(calc_cost(avg.floor() as i32), calc_cost(avg.ceil() as i32))
		}
	);
}