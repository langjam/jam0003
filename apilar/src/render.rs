use crate::world::World;
use std::fmt;

// display procedure based off https://oneorten.dev/blog/automata_rust_1/

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.rows {
            for location in row.iter() {
                let ch = if location.computer.is_some() {
                    '#'
                } else if location.resources > 5000 {
                    'X'
                } else if location.resources > 2000 {
                    'x'
                } else if location.resources > 0 {
                    '.'
                } else {
                    ' '
                };

                write!(f, "{}", ch)?;
            }

            write!(f, "\n")?;
        }
        write!(f, "Computers : {}\n", self.computers_amount())?;
        write!(f, "Processors: {}\n", self.processors_amount())?;
        Ok(())
    }
}

pub fn render_start() {
    print!("\x1b[2J\x1b[?25l");
}

pub fn render_update() {
    print!("\x1b[;H");
}
