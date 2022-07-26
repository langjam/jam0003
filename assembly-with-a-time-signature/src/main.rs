use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use clap::Arg;
use lwb_parser::sources::source_file::SourceFile;
use vm::Cpu;


use crate::desugar::desugar_ast;
use crate::elf::ElfFile;
use crate::midi::LiquidPiano;

use crate::parser::parse;

mod parser;
mod instruction_code;
mod desugar;
mod elf;
mod vm;
mod midi;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::command!()
        .name("masm")
        .arg(
            Arg::new("output")
                .short('o')
        )
        .arg(
            Arg::new("inputs")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::new("compile")
                .short('c')
                .takes_value(false)
                .help("stops before executing, produces object files as output")
        )
        .arg(
            Arg::new("instruments")
                .short('i')
                .takes_value(false)
                .help("shows you all available instruments")
        )
        .get_matches();

    let instruments = matches.is_present("instruments");
    if instruments {
        let instruments = midi::get_all_instruments();
        for i in instruments {
            println!("{}", i);
        }
        return Ok(());
    }
    let no_exec = matches.is_present("compile");
    let output = matches.value_of("output");
    let inputs: Vec<_> = matches.values_of("inputs")
        .expect("at least one input")
        .into_iter()
        .collect();

    if no_exec && output.is_some() && inputs.len() > 1 {
        return Err("can't specify an output file with -o when multiple input files are given in compile-only mode".into());
    }

    let assemble_tasks = if inputs.len() == 1 {
        let path = Path::new(inputs[0]);
        vec![
            (path, if no_exec && output.is_some() {
                Path::new(output.unwrap()).to_path_buf()
            } else {
                path.with_extension("o")
            })
        ]
    } else {
        inputs
            .iter()
            .map(Path::new)
            .filter(|i| i.extension() != Some(OsStr::new("o")))
            .map(|i| (i, i.with_extension("o")))
            .collect()
    };

    for (i, o) in &assemble_tasks {
        println!("assembling {:?}", i);
        let sf = SourceFile::open(i)?;
        let ast = parse(&sf);

        let instruction_stream = desugar_ast(ast);

        let mut f = ElfFile::create(o)?;
        f.write_unlinked(instruction_stream)?;
    }

    if !no_exec {
        println!("linking");
        let mut piano = LiquidPiano::new();

        let mut files = assemble_tasks
            .iter()
            .map(|i| i.1.as_path())
            .chain(
                inputs.iter()
                    .map(Path::new)
                    .filter(|i| i.extension() == Some(OsStr::new("o")))
            )
            .map(ElfFile::open)
            .collect::<Result<Vec<_>, _>>()?;

        let ctx = portmidi::PortMidi::new()?;

        let mut cpu = Cpu::new(&ctx);
        cpu.link(files.as_mut_slice())?;

        // for (_, i) in assemble_tasks {
        //     std::fs::remove_file(i)?;
        // }

        println!("running");
        loop {
            let (new_cpu, cont) = cpu.step()?;
            cpu = new_cpu;
            if !cont {
                break;
            }
            piano.check_running();
        }
    }

    Ok(())
}