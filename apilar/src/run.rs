use crate::assembler::Assembler;
use crate::computer::Computer;
use crate::render::{render_start, render_update};
use crate::world::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;

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
    words: Vec<&str>,
) {
    let assembler = Assembler::new();

    let mut computer = Computer::new(starting_memory_size, max_processors, starting_resources);
    assembler.assemble_words(words, &mut computer.memory, 0);
    computer.add_processor(0);

    let mut world = World::new(width, height, world_resources);
    world.set((width / 2, height / 2), computer);

    let mut small_rng = SmallRng::from_entropy();

    render_start();
    let mut i: u64 = 0;
    loop {
        let redraw = i % redraw_frequency == 0;
        let mutate = i % mutation_frequency == 0;

        world.update(&mut small_rng, instructions_per_update);
        if mutate {
            world.mutate(&mut small_rng, 5, 0);
        }
        if redraw {
            render_update();
            println!("{}", world);
        }
        i += 1;
    }
}
