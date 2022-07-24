use crate::assembler::{text_to_words, Assembler};
use crate::computer::Computer;
use crate::render::{render_start, render_update};
use crate::world::World;
use rand::rngs::SmallRng;
use rand::SeedableRng;

const PROGRAM_TEXT: &str = "
# startup
NOOP
NOOP
NOOP
NOOP
NOOP
NOOP  # delay so we take the right address after SPLIT
ADDR  # s
N6
SUB   # adjust s for delay
DUP   # s c
DUP   # s c c
N8
N8
MUL
ADD   # s c t target is 64 positions below start
SWAP  # s t c
# start copy loop
ADDR  # s t c l
EAT   # do some eating and growing while we can
GROW
SWAP  # s t l c
ROT   # s l c t
DUP2
ADD   # s l c t c+t
ROT   # s l t c+t c
DUP   # s l t c+t c c
READ  # s l t c+t c inst
ROT   # s l t c inst c+t
SWAP  # s l t c c+t inst
WRITE # s l t c
N1
ADD   # s l t c+1
ROT   # s t c+1 l
SWAP  # s t l c+1
DUP   # s t l c+1 c+1
ADDR  # end
N7
N3
MUL   # 21
ADD   # s t l c+1 c+1 end
LT    # s t l c+1 b
ROT   # s t c+1 b l
SWAP  # s t c+1 l b
JMPIF # s t c+1
DROP  # s t
OVER  # s t s
ADD   # s s+t
DUP   # s s+t s+t
START # s s+t spawn processor into copy
N2    
SUB   # s s+t-2 split_addr
RND   # random direction
SPLIT # split from s+t-2
JMP   # jump to first addr";

pub fn run() {
    let assembler = Assembler::new();
    let words = text_to_words(PROGRAM_TEXT);

    let mut computer = Computer::new(300, 10, 500);
    assembler.assemble_words(words, &mut computer.memory, 0);
    computer.add_processor(0);

    let mut world = World::new(70, 40, 400);
    world.set((30, 20), computer);

    let mut small_rng = SmallRng::from_entropy();

    render_start();
    let mut i = 0;
    loop {
        let redraw = i % 100000 == 0;
        let mutate = i % 100000 == 0;

        world.update(&mut small_rng, 10);
        if mutate {
            world.mutate(&mut small_rng, 5, 0);
        }
        if redraw {
            // for row in &world.rows {
            //     for location in row {
            //         if let Some(computer) = &location.computer {
            //             println!(
            //                 "Computer {} {}",
            //                 computer.processors.len(),
            //                 computer.resources
            //             );
            //             println!("{}", assembler.line_disassemble(&computer.memory.values))
            //         }
            //     }
            // }
            render_update();
            println!("{}", world);
        }
        i += 1;
    }
}
