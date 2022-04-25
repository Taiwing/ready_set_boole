fn sub_powerset(set: &[i32], template: u128)-> Vec<i32> {
	let mut shift = 0;
	let mut sub_powerset: Vec<i32> = vec![];

	sub_powerset.extend_from_slice(set);
	sub_powerset.retain(|_| {
		let keep = (template >> shift & 1) != 0;
		shift += 1;
		keep
	});
	sub_powerset
}

pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
	let mut powerset: Vec<Vec<i32>> = vec![];
	let mut template: u128 = 0;
	let end: u128;

	if set.len() >= 128 {
		panic!("input is waaaaay too big, please calm down");
	}
	end = 1 << set.len();
	while template < end {
		powerset.push(sub_powerset(set, template));
		template += 1;
	}
	powerset
}
