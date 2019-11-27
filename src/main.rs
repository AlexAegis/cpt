mod args;

use crate::args::FromArgs;
use cpt::model::Cpt;
use cpt::model::Vals;

#[cfg_attr(tarpaulin, skip)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	Cpt::<String, Vals>::from_args(None)?.execute()
}
