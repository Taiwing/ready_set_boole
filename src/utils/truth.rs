use std::collections::HashMap;
use std::io::{Write, BufWriter};

pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    if formula.len() == 0 {
        panic!("formula string is empty");
    }
    for op in formula.chars() {
        let right = if "01!".contains(op) { None } else { stack.pop() };
        let left = if "01".contains(op) { None } else { stack.pop() };
        match (op, left, right) {
            ('0', None, None) => stack.push(false),
            ('1', None, None) => stack.push(true),
            ('!', Some(a), None) => stack.push(!a),
            ('&', Some(a), Some(b)) => stack.push(a && b),
            ('|', Some(a), Some(b)) => stack.push(a || b),
            ('^', Some(a), Some(b)) => stack.push(a != b),
            ('>', Some(a), Some(b)) => stack.push(!a || b),
            ('=', Some(a), Some(b)) => stack.push(a == b),
            _ => panic!("'{}' is not a valid op or is missing an argument", op),
        }
    }
    if stack.len() > 1 {
        panic!("the stack should be empty at the end");
    }
    stack.pop().expect("nothing to return (stack is empty)")
}

fn write_truth<W: Write>(
	table: &mut String,
	writer: &mut Option<&mut BufWriter<W>>,
	truth: String
) {
	if let Some(w) = writer {
		w.write(truth.as_bytes()).unwrap();
	} else {
		table.push_str(&truth)
	};
}

fn build_truth(
	varmap: &HashMap<char, bool>,
	keys: &Vec<char>,
	is_header: bool
) -> String {
	let mut truth: String = String::with_capacity((27*4+1)*2);

    for key in keys.iter() {
        let value = *varmap.get(key).unwrap();
        let c = if is_header { *key } else if value { '1' } else { '0' };
        truth.push_str(&format!("| {} ", c));
    }
    truth.push_str("|");
    if is_header {
        let sep: String = truth.clone().chars()
            .map(|x| if x == '|' { '|' } else { '-' }).collect();
		format!("{}\n{}\n", truth, sep)
    } else { format!("{}\n", truth) }
}

fn find_truth(formula: &str, varmap: &mut HashMap<char, bool>) {
    let mut compiled_formula: String = String::with_capacity(formula.len());

    for c in formula.chars() {
        let op = match c {
            'A'..='Z' => { if *varmap.get(&c).unwrap() { '1' } else { '0' } },
            _ => c,
        };
        compiled_formula.push(op);
    }
    varmap.insert('=', eval_formula(&compiled_formula));
}

fn set_values(varmap: &mut HashMap<char, bool>, keys: &Vec<char>, values: u32) {
    let length: usize = keys.len() - 1;
    if values == 0 { return }
    for shift in 0..length {
        let value = (values >> shift) & 1 != 0;
        varmap.insert(keys[length - shift - 1], value);
    }
}

pub fn table<W: Write>(
	formula: &str,
	mut writer: Option<&mut BufWriter<W>>
) -> Option<String> {
    let mut varmap: HashMap<char, bool> = HashMap::with_capacity(27);
    let mut keys: Vec<char> = Vec::with_capacity(27);
	let mut table: String = String::new();
    let mut values: u32 = 0;

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                if varmap.insert(c, false) == None { keys.push(c); }
            },
            '0' | '1' => panic!("'{}' is not a valid op", c),
            _ => (),
        }
    }
    let values_max: u32 = 1 << keys.len();
    varmap.insert('=', false);
    keys.push('=');
    keys.sort();
    keys.rotate_left(1);
    write_truth(&mut table, &mut writer, build_truth(&varmap, &keys, true));
    loop {
        set_values(&mut varmap, &keys, values);
        find_truth(formula, &mut varmap);
        let truth = build_truth(&varmap, &keys, false);
		write_truth(&mut table, &mut writer, truth);
        values = values + 1;
        if values == values_max { break; }
    };
	Some(table)
}
