#![feature(iter_intersperse)]
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use clap::{Parser, Subcommand};

use anyhow::{anyhow, bail, Result};
use colored::Colorize;
use lazy_static::__Deref;
use rustyline::{
    error::ReadlineError, Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers,
};
use tafokr::{parse_str, Machine, MachineVerbosity};
use uxlol::RustylineExt;

mod uxlol;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Read Eval Print Execute
    Repl {},
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Repl {} => {
            let machine = Rc::new(RefCell::new(Machine::new()));
            {
                let mut m = machine.deref().borrow_mut();
                m.slot_in(parse_str(include_str!("./repl.taf"))?);
                m.eval(MachineVerbosity::Normal)?;
                assert!(m.data.is_empty(), "repl prelude should leave stack empty");
                // blackbox our builtins
                m.trace_pc_range = m.current_pc + 1..usize::MAX;
            }
            let ext = RustylineExt::new(Rc::downgrade(&machine));
            let mut rl = Editor::new()?;
            rl.set_helper(Some(ext));
            rl.load_history(".tafokr-history").ok(); // ignore err
            rl.bind_sequence(
                KeyEvent(KeyCode::Enter, Modifiers::CTRL),
                EventHandler::Simple(Cmd::Newline),
            );
            loop {
                let prompt = {
                    let m = machine.deref().borrow();
                    let data_len_unc = format!("{:02}", m.data.len());
                    let data_len = if m.data.is_empty() {
                        data_len_unc.green()
                    } else {
                        data_len_unc.yellow()
                    };

                    let sd_warn = if m.program_stack.len() > 1 {
                        format!(
                            "program stack depth ={}, be careful!",
                            m.program_stack.len()
                        )
                        .black()
                        .on_yellow()
                        .to_string()
                    } else {
                        "".to_string()
                    };

                    format!(
                        "{sd_warn}\n{:02} taf{}{:03}> ",
                        data_len,
                        "#".magenta(),
                        m.current_pc
                    )
                };

                match rl.readline(&prompt) {
                    Ok(line) => {
                        rl.add_history_entry(&line);
                        match repl_eval_line(&mut machine.borrow_mut(), &line) {
                            Ok(()) => {}
                            Err(err) => {
                                eprintln!("{}", err)
                            }
                        }
                    }
                    Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
                    Err(other) => {
                        bail!(other);
                    }
                }
            }
            rl.append_history(".tafokr-history")?;
        } // _ => unimplemented!()
    }
    Ok(())
}

fn repl_eval_line(machine: &mut Machine, mut line: &str) -> Result<()> {
    enum ReplTask {
        Parse,
        Eval,
    }

    if line.starts_with(":c") {
        machine.data.clear();
        eprint!("\x1Bc");
    }
    if line.starts_with(":d") || line.starts_with(":c") {
        eprintln!(
            "== ps ==\n{}",
            machine.program_stack.iter().map(|x| format!("{x:?}")).intersperse("\n".to_string()).collect::<String>()
        );
        eprintln!("{}", "—".repeat(50).white().bold());
        eprintln!("== data ==\n{}", machine.debug_formatv(&machine.data, tafokr::MachineDebugFormatLines::Multi)?);
        return Ok(());
    }

    let task = if line.starts_with(":p") {
        line = &line[2..];
        ReplTask::Parse
    } else {
        ReplTask::Eval
    };

    let toks = parse_str(line)?;

    match task {
        ReplTask::Parse => {
            eprintln!("{toks:#?}");
        }
        ReplTask::Eval => {
            machine.slot_in(toks);
            let dat_count = machine
                .eval(MachineVerbosity::Trace)
                .map_err(|er| anyhow!("Eval error:\n\t{er}"))?;
            if dat_count > 0 {
                let new = &machine.data[machine.data.len() - dat_count..];
                let mut buf = String::with_capacity(128);
                for tok in new {
                    machine.debug_format_inplace(tok, &mut buf)?;
                    buf.push('\n');
                }
                eprintln!("{buf}");
            }
        }
    }
    Ok(())
}
