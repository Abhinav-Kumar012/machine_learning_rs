use std::{error::Error, fs::File};

use csv::ReaderBuilder;

const DSET_PATH: &str = "data/titanic";

#[derive(Debug, serde::Deserialize)]
enum sex {
	#[serde(rename = "male")]
	male,
	#[serde(rename = "female")]
	female,
}

#[derive(Debug, serde::Deserialize)]
enum port {
	#[serde(rename = "C")]
	Cherbourg,
	#[serde(rename = "Q")]
	Queenstown,
	#[serde(rename = "S")]
	Southampton,
	#[serde(other)]
	Missing,
}

#[derive(Debug, serde::Deserialize)]
struct RawRecord {
	// #[serde(rename(deserialize = "PassengerId"))]
	// rec_no: u32,
	#[serde(rename(deserialize = "Survived"))]
	survived: u8,
	#[serde(rename(deserialize = "Pclass"))]
	passenger_class: u8,
	// #[serde(rename(deserialize = "Name"))]
	// name: String,
	#[serde(rename(deserialize = "Sex"))]
	sex: sex,
	#[serde(rename(deserialize = "Age"))]
	age: Option<f32>,
	#[serde(rename(deserialize = "SibSp"))]
	sib_sp: u8,
	#[serde(rename(deserialize = "Parch"))]
	parch: u8,
	#[serde(rename(deserialize = "Ticket"))]
	ticket: String,
	#[serde(rename(deserialize = "Fare"))]
	fare: f64,
	// #[serde(rename(deserialize = "Cabin"))]
	// cabin: String,
	#[serde(rename(deserialize = "Embarked"))]
	embark: port,
}

fn main() -> Result<(), Box<dyn Error>> {
	let file_path = format!("{DSET_PATH}/train.csv");

	let fd = File::open(file_path)?;

	let mut reader = ReaderBuilder::new()
		.delimiter(b',')
		.has_headers(true)
		.quote(b'\"')
		.from_reader(fd);

	for result in reader.deserialize() {
		let record: RawRecord = result?;
		println!("{record:?}");
	}
	Ok(())
}
