mod args;

use crate::args::FromArgs;
use cpt::model::{Cpt, StringOrVecString};

#[cfg_attr(tarpaulin, skip)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	Cpt::<String>::from_args(None)?.execute()
}
