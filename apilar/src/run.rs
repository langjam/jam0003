use crate::assembler::Assembler;
use crate::computer::Computer;
use crate::render::{render_start, render_update};
use crate::world::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::error::Error;
use std::fs::File;

pub fn run(
    width: usize,
    height: usize,
    starting_memory_size: usize,
    starting_resources: u64,
    max_processors: usize,
    world_resources: u64,
    instructions_per_update: usize,
    mutation_frequency: u64,
    redraw_frequency: u64,
    save_frequency: u64,
    memory_mutation_amount: u64,
    processor_stack_mutation_amount: u64,
    eat_amount: u64,
    dump: bool,
    words: Vec<&str>,
) -> Result<(), Box<dyn Error>> {
    let assembler = Assembler::new();

    let mut computer = Computer::new(starting_memory_size, max_processors, starting_resources);
    assembler.assemble_words(words, &mut computer.memory, 0);
    computer.add_processor(0);

    let mut world = World::new(width, height, eat_amount, world_resources);
    world.set((width / 2, height / 2), computer);

    let mut small_rng = SmallRng::from_entropy();

    render_start();
    let mut i: u64 = 0;
    let mut save_nr = 0;

    loop {
        let redraw = i % redraw_frequency == 0;
        let mutate = i % mutation_frequency == 0;
        let save = i % save_frequency == 0;

        world.update(&mut small_rng, instructions_per_update);
        if mutate {
            world.mutate(
                &mut small_rng,
                memory_mutation_amount,
                processor_stack_mutation_amount,
            );
        }
        if redraw {
            render_update();
            println!("{}", world);
        }
        if save && dump {
            let file = File::create(format!("apilar-dump{}.cbor", save_nr))?;
            serde_cbor::to_writer(file, &world)?;
            save_nr += 1;
        }
        i = i.wrapping_add(1);
    }
}
