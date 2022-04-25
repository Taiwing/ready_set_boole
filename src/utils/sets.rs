use std::collections::HashMap;

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

fn build_varmap(formula: &str, sets: &[Vec<i32>]) -> HashMap<char, Vec<i32>> {
	let mut index = 0;
	let mut keys: Vec<char> = Vec::with_capacity(26);
	let mut varmap: HashMap<char, Vec<i32>> = HashMap::with_capacity(26);

	for c in formula.chars().filter(|c| {
		match c { 'A'..='Z' => true, _ => false }
	}) {
		if keys.contains(&c) == false {
			keys.push(c);
		}
	}
	keys.sort();
	if keys.len() > sets.len() {
		panic!("missing set operands for '{}' formula", formula);
	} else if sets.len() > keys.len() {
		panic!("too many set operands for '{}' formula", formula);
	}
	for c in keys {
		let mut set = sets[index].clone();
		set.sort();
		varmap.insert(c, set);
		index += 1;
	}
	varmap
}

fn set_complement(a: &Vec<i32>, u: &Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	for element in u {
		if a.contains(element) == false {
			result.push(*element);
		}
	}
	result.sort();
	result
}

fn set_intersection(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	for element in a {
		if b.contains(element) {
			result.push(*element);
		}
	}
	result.sort();
	result
}

fn set_union(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	result.append(&mut a.clone());
	for element in b {
		if result.contains(element) == false {
			result.push(*element);
		}
	}
	result.sort();
	result
}

fn set_xor(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	for element in a {
		if b.contains(element) == false {
			result.push(*element);
		}
	}
	for element in b {
		if a.contains(element) == false {
			result.push(*element);
		}
	}
	result.sort();
	result
}

fn set_implication(a: &Vec<i32>, b: &Vec<i32>, u: &Vec<i32>) -> Vec<i32> {
	set_union(&set_complement(a, u), b)
}

fn set_equal(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
	let mut result: Vec<i32> = vec![];

	if a != b {
		return result
	}
	result.append(&mut a.clone());
	result.sort();
	result
}

pub fn eval_set(formula: &str, sets: &[Vec<i32>]) -> Vec<i32> {
	let mut u: Vec<i32> = Vec::new();
    let mut stack: Vec<Vec<i32>> = Vec::new();
	let variables: String = ('A'..='Z').collect();

    if formula.len() == 0 {
        panic!("formula string is empty");
    }
	let varmap = build_varmap(formula, sets);
	for set in sets {
		for element in set {
			if u.contains(element) == false {
				u.push(*element);
			}
		}
	}
	u.sort();
    for op in formula.chars() {
        let right = if op == '!' || variables.contains(op) {
			None
		} else {
			stack.pop()
		};
        let left = if variables.contains(op) { None } else { stack.pop() };
        match (op, left, right) {
            ('A'..='Z', None, None) => {
				stack.push(varmap.get(&op).unwrap().clone())
			},
            ('!', Some(a), None) => stack.push(set_complement(&a, &u)),
            ('&', Some(a), Some(b)) => stack.push(set_intersection(&a, &b)),
            ('|', Some(a), Some(b)) => stack.push(set_union(&a, &b)),
            ('^', Some(a), Some(b)) => stack.push(set_xor(&a, &b)),
            ('>', Some(a), Some(b)) => stack.push(set_implication(&a, &b, &u)),
            ('=', Some(a), Some(b)) => stack.push(set_equal(&a, &b)),
            _ => panic!("'{}' is not a valid op or is missing an argument", op),
        }
    }
    if stack.len() > 1 {
        panic!("the stack should be empty at the end");
    }
    stack.pop().expect("nothing to return (stack is empty)")
}
