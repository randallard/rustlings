// #![allow(unused)]

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

// build from start using https://youtu.be/oxx7MmN4Ib0?si=CICOMZS2r9yQmgCy
// continue from https://youtu.be/oxx7MmN4Ib0?si=9-_CGITvcHfT1TAP&t=232

fn main() -> Result<()> {
    println!("Hello, world!");

    for entry in read_dir("../../")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }

    Ok(())
}
