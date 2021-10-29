
use micro_elf::*;

use std::io::prelude::*;

fn main() {
	let path = std::env::current_dir().unwrap().join("test.elf");

	println!("{:?}", path.display());

	let file = std::fs::File::open(path).unwrap();

	let mut data = Vec::new();

	let mut reader = std::io::BufReader::new(file);

	reader.read_to_end(&mut data).unwrap();

	let elf: Elf<X32> = Elf::parse(data);

	println!("{}", elf);
}