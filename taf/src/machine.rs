use std::collections::HashMap;

use anyhow::{anyhow, bail as anyhow_bail, Result};
use colored::Colorize;
use std::ops::Range;

use crate::{Point, Tok};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Our machine can do more than just execute instructions linearly.
pub enum HybridPc {
    Step1 { ret: usize, point: Option<String> },
    WalkToEnd,
}

#[derive(Debug, Clone)]
pub enum MachineDebugFormatLines {
    Single,
    Multi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineVerbosity {
    Normal,
    Trace,
}

#[derive(Debug)]
pub struct Machine {
    pub points: HashMap<String, Point>,
    pub data: Vec<Tok>,
    pub program: Vec<Tok>,
    /// A stack of "program counters". You ~ (Tok::End) to pop and ! (Tok::Call) to push
    pub program_stack: Vec<HybridPc>,
    pub current_pc: usize,
    /// Only print trace line if within this bit of the
    /// program_stack.
    pub trace_pc_range: Range<usize>,
}

impl Machine {
    fn last_step1(&self) -> Option<(usize, Option<String>)> {
        for pc in self.program_stack.iter().rev() {
            if let HybridPc::Step1 { ret, point } = (*pc).clone() {
                return Some((ret, point));
            }
        }
        None
    }

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("print".to_string(), Point::Print);
        map.insert("+".to_string(), Point::Add);
        map.insert("-".to_string(), Point::Dec);
        map.insert("dupn".to_string(), Point::Dupn);
        map.insert(".".to_string(), Point::Spot);
        map.insert(">".to_string(), Point::Gt);
        Machine {
            points: map,
            data: vec![],
            program: vec![],
            program_stack: vec![HybridPc::Step1 {
                ret: 0,
                point: None,
            }],
            current_pc: 0,
            trace_pc_range: 0..usize::MAX,
        }
    }
    /// Push a program into the machine. Very skeuomorphic.
    pub fn slot_in(&mut self, mut a_program: Vec<Tok>) {
        self.program.append(&mut a_program);
    }

    /// Evaluates the program in the machine and returns the amount
    /// of data pushed to the stack by this evaluation.
    pub fn eval(&mut self, verb: MachineVerbosity) -> Result<usize> {
        // Only use to track how much we've pushed.
        // We're allowed to use data on the stack from
        // before.
        let mut pushed: usize = 0;

        macro_rules! bail {
            ($msg:literal) => {
                bail!($msg, )
            };
            // ($err:expr $(,)?) => {
            //     bail!($err);
            // };
            ($fmt:expr, $($arg:tt)*) => {{
                if let Some((ret, Some(point))) = self.last_step1() {
                    eprintln!("while calling #{point}:");
                    self.program_stack.pop().unwrap();
                    self.current_pc = ret;
                }
                anyhow_bail!($fmt, $($arg)*)
            }};
        }
        macro_rules! push {
            ($e:expr) => {
                self.data.push($e);
                pushed += 1;
            };
        }
        macro_rules! pop {
            ($match:pat,$ret:expr,$thing:expr) => {
                match (pushed, self.data.pop()) {
                    (_, Some($match)) => {
                        pushed = pushed.saturating_sub(1);
                        $ret
                    }
                    (_, _) => bail!(
                        "cannot pop{} from an {} (tok={})",
                        $thing,
                        "empty stack".magenta(),
                        self.debug_format(&self.program[self.current_pc - 1])?
                    ),
                }
            };
            () => {
                pop!(tok, tok, "")
            };
            ($i:ident) => {
                pop!(Tok::$i(x), x, format!(" {}", stringify!($i).magenta()))
            };
        }

        while self.current_pc < self.program.len() {
            let tok = self.program.get(self.current_pc).unwrap();
            let dbg_start = if verb == MachineVerbosity::Trace {
                format!(
                    "{ps}\t\t{s}{tok}{s}",
                    s = "|".white(),
                    tok = self.debug_format(tok)?.on_black(),
                    ps = {
                        match self.program_stack.last().unwrap() {
                            HybridPc::Step1 { ret, point } => format!(
                                "#{:4}{ret:03}",
                                match point {
                                    Some(p) => p.to_string(),
                                    None => "_".to_string(),
                                }
                            ),
                            HybridPc::WalkToEnd => "————————>".to_string(),
                        }
                    }
                )
            } else {
                "".to_string()
            };
            self.current_pc += 1;
            // dbg!(&self);
            match self.program_stack.last().unwrap() {
                HybridPc::WalkToEnd => match tok {
                    Tok::End => {
                        self.program_stack.pop();
                    }
                    _ => {}
                },

                HybridPc::Step1 { ret, point: _ } => match tok {
                    // <literals boring=true>
                    Tok::Ptr(id) => {
                        push!(Tok::Ptr(id.clone()));
                    }
                    Tok::String(st) => {
                        push!(Tok::String(st.clone()));
                    }
                    Tok::Int(i) => {
                        push!(Tok::Int(*i));
                    }
                    // </literals>
                    Tok::Point(st) => {
                        self.points.insert(st.clone(), Point::User(self.current_pc));
                        self.program_stack.push(HybridPc::WalkToEnd);
                    }
                    // Pop up the stack.
                    Tok::End => {
                        // some rust tomfoolery. For some reason copy semantics didn't happen on the HybridPc::Linear(ret_pc) pattern above
                        let ret_pc_clone = *ret;
                        if self.program_stack.len() == 1 {
                            bail!("cannot pop up any further");
                        }
                        self.program_stack.pop().unwrap();
                        self.current_pc = ret_pc_clone;
                    }
                    Tok::Call => {
                        let ptrptr = &pop!(Ptr);
                        let ptr = match self.points.get(ptrptr) {
                            Some(ptr) => ptr,
                            None => bail!("{ptrptr} is not defined"),
                        };
                        match ptr {
                            Point::User(point) => {
                                // Where to return to.

                                // TCO could be done here but this isn't a real CPU and we have a looot of memory!
                                self.program_stack.push(HybridPc::Step1 {
                                    ret: self.current_pc,
                                    point: Some(ptrptr.clone()),
                                });
                                let max_depth = 100_000;
                                if self.program_stack.len() > max_depth {
                                    self.program_stack.clear();
                                    bail!("maximum stack depth () reached. cleared.");
                                }
                                // Where we'll go next.
                                self.current_pc = *point;
                            }
                            Point::Gt => {
                                let y = pop!(Int);
                                let x = pop!(Int);
                                if x > y {
                                    self.current_pc += 2; // HACk
                                }
                            }
                            Point::Print => {
                                println!("{}", pop!(String));
                            }
                            Point::Add => {
                                let a = pop!(Int);
                                let b = pop!(Int);
                                push!(Tok::Int(a + b));
                            }
                            Point::Dec => {
                                let a = pop!(Int);
                                let b = pop!(Int);
                                push!(Tok::Int(b - a));
                            }
                            Point::Dupn => {
                                let n = pop!(Int);
                                let x = pop!();
                                for _i in 0..n {
                                    push!(x.clone());
                                }
                                push!(x);
                            }
                            // Swap the last element and another one indexed by last-n
                            Point::Spot => {
                                // Call:
                                //  1 2 3 1 #.!
                                let arg = pop!(Int);
                                // i0 1 2
                                //  1 2 3
                                let idx = (self.data.len() - (arg as usize)) - 1;
                                //    ^
                                let a = self
                                    .data
                                    .get(idx)
                                    .ok_or_else(|| anyhow!("spot is trying to index too far back"))?
                                    .clone();
                                let b = pop!();
                                push!(a);
                                *self.data.get_mut(idx).unwrap() = b;
                            }
                        }
                    }
                    #[allow(unreachable_patterns)]
                    _ => bail!("{tok:?} is unimplemented"),
                },
            }
            if verb == MachineVerbosity::Trace && self.trace_pc_range.contains(&self.current_pc) {
                eprintln!(
                    "{depth}{dbg_start}{:pad$}{dat}",
                    pad = 60_usize.saturating_sub(dbg_start.len()),
                    depth = "\t".repeat(self.program_stack.len() - 1),
                    dat = self
                        .debug_formatv(&self.data, MachineDebugFormatLines::Single)?
                        .cyan()
                );
            }

            // dbg!(&self);
        }
        Ok(pushed)
    }

    pub fn debug_format_inplace(&self, t: &Tok, buf: &mut String) -> Result<()> {
        match t {
            Tok::String(s) => {
                buf.push('"');
                buf.push_str(s);
                buf.push('"');
            }
            Tok::Ptr(ptrptr) => {
                let ptr = self
                    .points
                    .get(ptrptr)
                    .map_or_else(|| "<undefined>".to_string(), |p| format!("{p:?}"));
                buf.push_str(&format!("#{ptrptr} ({ptr})"));
            }
            Tok::Int(i) => buf.push_str(&format!("{i}")),
            Tok::Call => buf.push('!'),
            Tok::Point(p) => {
                buf.push_str(&format!("@{p}"));
            }
            #[allow(unreachable_code)]
            _ => {
                buf.push_str(&format!("{t:#?}"));
            }
        }
        Ok(())
    }

    pub fn debug_format(&self, t: &Tok) -> Result<String> {
        let mut buf = String::with_capacity(64);
        self.debug_format_inplace(t, &mut buf)?;
        Ok(buf)
    }

    pub fn debug_formatv(&self, tv: &[Tok], l: MachineDebugFormatLines) -> Result<String> {
        let mut buf = String::with_capacity(256);
        for (i, t) in tv.iter().enumerate() {
            let _next = &tv[(i + 1) % tv.len()];
            buf.push_str(if t != &Tok::Call { "\t" } else { " " });
            self.debug_format_inplace(t, &mut buf)?;
            if i != tv.len() - 1 {
                let next = &tv[(i + 1) % tv.len()];
                if next != &Tok::Call {
                    buf.push_str(match l {
                        MachineDebugFormatLines::Single => ", ",
                        MachineDebugFormatLines::Multi => "\n",
                    });
                }
            }
        }
        Ok(buf)
    }
}
