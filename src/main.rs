#![warn(clippy::all)]

extern crate rustcalc;

use rustcalc::cli::CLI;
use rustcalc::error::Error;

fn main() -> Result<(), Error> {
    CLI::new().run()
}
