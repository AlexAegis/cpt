use std::error::Error;

pub mod args;
pub mod model;
pub use model::Cpt;

pub fn cp(from: String, to: String) -> Result<(), Box<dyn Error>> {
	Cpt::new(from, to).execute()
}

pub fn cpt(from: String, to: String, data: serde_json::Value) -> Result<(), Box<dyn Error>> {
	Cpt::new(from, to).set_data(data).execute()
}
