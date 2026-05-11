use std::{error::Error, fs::File};

use csv::Reader;

const DSET_PATH: &str = "data/titanic";

fn main() -> Result<(), Box<dyn Error>> {
	let file_path = format!("{DSET_PATH}/train.csv");

	let fd = File::open(file_path)?;

	let mut reader = Reader::from_reader(fd);

	for result in reader.records() {
		println!("{:?}", result?);
	}
	Ok(())
}
