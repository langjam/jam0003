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
pub mod starter;
pub mod world;

#[cfg(test)]
pub mod testutil;

use crate::assembler::text_to_words;
use crate::run::run;
use crate::starter::PROGRAM_TEXT;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    filename: Option<String>,

    #[clap(long, value_parser)]
    width: Option<usize>,

    #[clap(long, value_parser)]
    height: Option<usize>,

    #[clap(long, value_parser)]
    starting_memory_size: Option<usize>,

    #[clap(long, value_parser)]
    starting_resources: Option<u64>,

    #[clap(long, value_parser)]
    max_processors: Option<usize>,

    #[clap(long, value_parser)]
    world_resources: Option<u64>,

    #[clap(long, value_parser)]
    instructions_per_update: Option<usize>,

    #[clap(long, value_parser)]
    mutation_frequency: Option<u64>,

    #[clap(long, value_parser)]
    redraw_frequency: Option<u64>,

    #[clap(long, value_parser)]
    memory_mutation_amount: Option<u64>,

    #[clap(long, value_parser)]
    processor_stack_mutation_amount: Option<u64>,

    #[clap(long, value_parser)]
    eat_amount: Option<u64>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let contents = match cli.filename {
        Some(filename) => {
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            contents
        }
        None => PROGRAM_TEXT.to_string(),
    };
    let words = text_to_words(&contents);

    run(
        cli.width.unwrap_or(70),
        cli.height.unwrap_or(40),
        cli.starting_memory_size.unwrap_or(300),
        cli.starting_resources.unwrap_or(500),
        cli.max_processors.unwrap_or(10),
        cli.world_resources.unwrap_or(400),
        cli.instructions_per_update.unwrap_or(10),
        cli.mutation_frequency.unwrap_or(100000),
        cli.redraw_frequency.unwrap_or(100000),
        cli.memory_mutation_amount.unwrap_or(5),
        cli.processor_stack_mutation_amount.unwrap_or(0),
        cli.eat_amount.unwrap_or(100),
        words,
    );
    Ok(())
}
