use std::error::Error;

use polars::{
	lazy::frame::{LazyCsvReader, LazyFileListReader},
	prelude::PlRefPath,
};

const DSET_PATH: &str = "data/titanic";

fn run() -> Result<(), Box<dyn Error>> {
	let ldf = LazyCsvReader::new(PlRefPath::new(format!("{}/train.csv", DSET_PATH)))
		.with_has_header(true)
		.finish()?;
	print!("{:#?}",ldf.collect()?);

	Ok(())
}

fn main() {
	match run() {
		Ok(()) => {
			println!("Done")
		}
		Err(e) => {
			eprintln!("{e}");
		}
	};
}
