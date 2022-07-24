use crate::assembler::{text_to_words, Assembler};
use crate::computer::Computer;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

// demonstrates replication within a single computer

pub fn main() -> std::io::Result<()> {
    let assembler = Assembler::new();

    let text = "
        # startup
        ADDR  # s
        DUP   # s c
        DUP   # s c c
        RND   # 
        ADD   # s c t
        RND   
        ADD   # s c t
        SWAP  # s t c
        # start copy loop
        ADDR  # s t c l
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
        ADDR
        N8
        N8
        N4
        ADD
        ADD   # add to get end of replicator
        ADD   # s t l c+1 c+1 end
        LT    # s t l c+1 b
        ROT   # s t c+1 b l
        SWAP  # s t c+1 l b
        JMPIF # s t c+1
        DROP  # s t
        OVER  # s t s
        ADD   # s s+t
        START # s
        JMP   # jump to first addr
        ";
    let words = text_to_words(text);

    let mut computer = Computer::new(1024 * 1024, 100, 100);
    assembler.assemble_words(words.clone(), &mut computer.memory, 0);
    // let mut small_rng = SmallRng::from_seed([0; 32]);
    let mut small_rng = SmallRng::from_entropy();

    computer.add_processor(0);

    let mut i = 0;
    let mut total = 0;
    let mut dump_count = 0;
    loop {
        total += computer.execute(&mut small_rng, 100);
        if total > 5000 {
            println!("Processors {}", computer.processors.len());
            let words = assembler.disassemble_to_words(&computer.memory.values);

            let file = File::create(format!("dump{}.apil", dump_count))?;
            let mut stream = BufWriter::new(file);
            dump_count += 1;
            for word in words {
                stream.write(word.as_bytes())?;
                stream.write("\n".as_bytes())?;
            }
            stream.flush()?;
            println!("Written");
            computer.mutate_memory(&mut small_rng, 1);
            computer.mutate_processors(&mut small_rng, 1);
            total = 0;
        }
        // mutation should be somehow relative to instructions executed, not
        // steps in this loop. it's not quite there yet as the total is increasing
        // by huge steps. Perhaps I should sample processors
        // if i % 1000000 == 0 {
        //     computer.mutate_processors(&mut small_rng, 1);
        // }
        i += 1;
    }

    // let words = assembler.disassemble_to_words(&computer.memory.values);

    // let mut file = File::create("dump.apil")?;
    // for word in words {
    //     file.write(word.as_bytes())?;
    //     file.write("\n".as_bytes())?;
    // }
    Ok(())
}
