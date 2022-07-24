use std::collections::HashMap;
use std::io;
use std::fmt;

#[derive(Debug)]
struct SrcError {
    message: String,
}

impl fmt::Display for SrcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for SrcError {
    fn from(error: io::Error) -> Self {
        SrcError {
            message: error.to_string(),
        }
    }
}

#[derive(Debug)]
enum Command {
    Sequence(Vec<i64>),
    Selection(Vec<i64>),
    Iteration(Vec<i64>),
}

struct Evaluator {
    vars: HashMap<i64, i64>,
    answer: i64,
}

fn value_sequence(seq_param: &[i64]) -> Result<i64, SrcError> {
    match seq_param.first() {
        Some(x) => Ok(*x),
        None => return Err(SrcError {message: String::from("Not an i64")}),
    }
}

impl Evaluator {
    fn new() -> Evaluator {
        Self {
            vars: HashMap::new(),
            answer: 0,
        }
    }

    fn follow_sequence(self, seq_params: &[i64], commands: &[Command]) -> Result<(), SrcError> {
        for seq in seq_params {
            match seq {
                x => self.evaluate(commands[x]),
                _ => return Err(SrcError {message: String::from("Not an i64")}),
            }
        }
        Ok(())
    }

    fn eval_selection() -> Result<(), SrcError> {
        Ok(())
    }

    fn eval_iteration() -> Result<(), SrcError> {
        Ok(())
    }

    fn start(mut self, commands: &[Command]) -> Result<i64, SrcError> {
        match commands.first().unwrap() {
            Command::Sequence(seq_vec) => {
                println!("{}", seq_vec[0]);
                self.follow_sequence(seq_vec, commands);
                self.answer = 5;
            },
            Command::Selection(_) => {
                return Err(SrcError {message: String::from("First command must be a SEQ")});
            },
            Command::Iteration(_) => {
                return Err(SrcError {message: String::from("First command must be a SEQ")});
            },
        }
        
        Ok(self.answer)
    }

    fn evaluate(mut self, command: &Command,  commands: &[Command]) -> Result<(), SrcError> {
            match command {
                Command::Sequence(seq_vec) => {
                    if seq_vec.len() > 1 {
                        self.follow_sequence(seq_vec, commands);
                    } else if seq_vec.len() == 1 {
                        self.answer += value_sequence(seq_vec)?;
                    } else {
                        return Err(SrcError {message: String::from("SEQ has no params")});
                    }
                    println!("{}", seq_vec[0]);
                },
                Command::Selection(sel_vec) => {
                    println!("{}", sel_vec[0]);
                },
                Command::Iteration(itr_vec) => {
                    println!("{}", itr_vec[0]);
                },
            }
        Ok(())
    }
}

fn parse_int(input: &str) -> Result<i64, SrcError> {
    let result = input.parse::<i64>();

    match result {
        Ok(x) => Ok(x),
        Err(error) => {
            return Err(SrcError {message: error.to_string()});
        },
    }
}

fn parse_params(param_strings: &[&str]) -> Result<Vec<i64>, SrcError> {
    Ok(param_strings.iter().map(|s| parse_int(s).unwrap()).collect())
}

fn parse_seq(input: &[&str]) -> Result<Command, SrcError> {
    if input.len() <= 1 {
        return Err(SrcError {message: String::from("Missing Operands for SEQ")});
    }

    let params = parse_params(input.split_first().unwrap().1)?;

    println!("Seq params {:?}", &params);

    Ok(Command::Sequence(params))
}

fn parse_sel(input: &[&str]) -> Result<Command, SrcError> {
    if input.len() <= 1 {
        return Err(SrcError {message: String::from("Missing Operands for SEQ")});
    }

    let params = parse_params(input.split_first().unwrap().1)?;

    Ok(Command::Selection(params))
}

fn parse_itr(input: &[&str]) -> Result<Command, SrcError> {
    if input.len() <= 1 {
        return Err(SrcError {message: String::from("Missing Operands for SEQ")});
    }

    let params = parse_params(input.split_first().unwrap().1)?;

    Ok(Command::Iteration(params))
}

fn parse(input: &str) -> Result<Vec<Command>, SrcError> {
    let mut output = vec![];

    for line in input.lines() {
        let command: Vec<_> = line.split_whitespace().collect();

        match command.first() {
            Some(x) if *x == "SEQ" => {
                output.push(parse_seq(&command)?);
            },
            Some(x) if *x == "SEL" => {
                output.push(parse_sel(&command)?);
            },
            Some(x) if *x == "ITR" => {
                output.push(parse_itr(&command)?);
            },
            Some(name) => {
                let mut s = String::from("Unknown command: ");
                s.push_str(*name);
                return Err(SrcError{message: s});
            },
            None => {
                return Err(SrcError{message: String::from("Empty line")});
            }
        }
    }

    Ok(output)
}

fn main() -> Result<(), SrcError> {
    for arg in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(arg).unwrap();
        let engine = Evaluator::new();
        let commands = parse(&contents)?;
        let answer = engine.start(&commands)?;
        // println!("{}", contents);
        println!("{:?}", commands);
        println!("{}", answer);
    }

    Ok(())
}

#[test]
fn test1() -> Result<(), SrcError> {

    Ok(())
}