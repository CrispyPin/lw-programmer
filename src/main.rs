use enigo::*;
use std::{env, fs, process::exit, thread, time::Duration};

const WRITE_KEY: Key = Key::Layout('j');
const BIT_KEYS: [Key; 16] = [
	Key::Layout('0'),
	Key::Layout('1'),
	Key::Layout('2'),
	Key::Layout('3'),
	Key::Layout('4'),
	Key::Layout('5'),
	Key::Layout('6'),
	Key::Layout('7'),
	Key::Layout('8'),
	Key::Layout('9'),
	Key::Layout('a'),
	Key::Layout('b'),
	Key::Layout('c'),
	Key::Layout('d'),
	Key::Layout('e'),
	Key::Layout('f'),
];

fn main() {
	let path = env::args().skip(1).next().unwrap_or_else(|| {
		println!("no path specified");
		exit(0)
	});
	let file = fs::read_to_string(path).unwrap();
	thread::sleep(Duration::from_millis(2000));

	let mut eni = Enigo::new();

	for line in file.lines() {
		let mut keys = Vec::new();
		let mut index = 16;
		for c in line.chars() {
			if c == '0' {
				index -= 1;
			} else if c == '1' {
				index -= 1;
				keys.push(BIT_KEYS[index]);
			}
			if index == 0 {
				break;
			}
		}
		for k in keys.clone() {
			eni.key_down(k);
		}
		eni.key_down(WRITE_KEY);
		thread::sleep(Duration::from_millis(50));
		eni.key_up(WRITE_KEY);
		thread::sleep(Duration::from_millis(50));
		for k in keys {
			eni.key_up(k);
		}
		thread::sleep(Duration::from_millis(50));
	}
}
