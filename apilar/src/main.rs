extern crate num;
#[macro_use]
extern crate num_derive;

pub mod assembler;
pub mod computer;
pub mod direction;
pub mod instruction;
pub mod memory;
pub mod processor;
pub mod render;
pub mod run;
pub mod single;
pub mod world;

#[cfg(test)]
pub mod testutil;

use crate::run::run;

fn main() -> std::io::Result<()> {
    run();
    Ok(())
}
