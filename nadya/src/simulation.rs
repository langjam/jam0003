use std::collections::{HashMap, HashSet};

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub value: i32,
    spawner: Option<Point>,
}

impl Variable {
    pub fn reset(&mut self) {
        self.value = 1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Simulation {
    variables: HashMap<Point, Variable>,
    program: Program,
    pub outputs: Vec<String>,
}

impl Simulation {
    pub fn find_variable(&self, position: Point) -> Option<&Variable> {
        self.variables.get(&position)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Spawner {
    Integer(i32),
    File { data: Vec<i32>, location: usize },
}

impl Spawner {
    pub fn spawn(&mut self) -> i32 {
        match self {
            Spawner::Integer(i) => *i,
            Spawner::File {
                data,
                ref mut location,
            } => {
                let num = data[*location];
                *location += 1;
                *location %= data.len();

                num
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimulationStateChange {
    Move {
        variable_point: Point,
        new_position: Point,
    },
    Merge {
        variable_points: Vec<Point>,
        new_position: Point,
    },
    Spawn {
        spawner: Point,
    },
    Kill {
        variable_point: Point,
    },
    DoNothing,
}

impl Simulation {
    pub fn new(mut program: Program) -> Self {
        // Spawn a variable at each entrypoint
        let variables = program
            .spawners
            .iter_mut()
            .map(|(point, spawner)| {
                (
                    *point,
                    Variable {
                        value: spawner.spawn(),
                        spawner: Some(*point),
                    },
                )
            })
            .collect();

        Self {
            variables,
            program,
            outputs: Vec::new(),
        }
    }

    pub fn map_string(&self) -> (String, i32, i32) {
        let mut output = String::new();

        for y in self.program.bounds.min_y..=self.program.bounds.max_y {
            for x in self.program.bounds.min_x..=self.program.bounds.max_x {
                // Check if a variable is at this point
                if let Some(_) = self.find_variable(Point { x, y }) {
                    // If so, represent it with an O
                    output.push_str("O");
                } else {
                    let point = Point { x, y };
                    let place = self.program.file.get(&point).unwrap();

                    // If it's a floor, use a space instead of a period
                    let symbol = match place.syntax {
                        Syntax::Floor => ' ',
                        _ => place.syntax.get_symbol(),
                    };

                    output.push(symbol);
                }
            }
            output.push('\n');
        }

        (
            output,
            self.program.bounds.max_x - self.program.bounds.min_x + 1,
            self.program.bounds.max_y - self.program.bounds.min_y + 1,
        )
    }

    pub fn simulate(&mut self) {
        // List to queue all changes this update
        let mut changes: Vec<SimulationStateChange> = Vec::new();

        // Track any variables that have already been assessed this update
        let mut already_assessed: HashSet<Point> = HashSet::new();

        // Move variables to the next location that their place points to
        self.variables.iter().for_each(|(point, _variable)| {
            // If we've already assessed this variable, skip it
            if already_assessed.contains(point) {
                return;
            }

            // Add this point to the list of already assessed points
            already_assessed.insert(*point);

            // If the variable is at the exit, move it back to the start
            if *point == self.program.exit {
                changes.push(SimulationStateChange::Kill {
                    variable_point: *point,
                });
            }

            let next_position = self
                .program
                .file
                .get(&point)
                .unwrap()
                .next
                .expect("Couldn't figure out where to go next");

            // Check the next position
            let next_place = self.program.file.get(&next_position).unwrap();

            // If there are multiple ways to get to this place, make sure other
            // each other position has a variable in it
            if next_place.prev.len() > 1 {
                // See if there's a variable in each position that leads into
                // this one
                if next_place
                    .prev
                    .iter()
                    .all(|point| self.find_variable(point.unwrap()).is_some())
                {
                    // Collect all of the variables that will be merging
                    let variables = next_place
                        .prev
                        .iter()
                        .map(|point| {
                            let point = point.unwrap();

                            let variable = self.find_variable(point).unwrap();

                            if variable.spawner.is_some() {
                                // Spawn new variables in their spawner's locations
                                changes.push(SimulationStateChange::Spawn {
                                    spawner: variable.spawner.unwrap(),
                                });
                            }

                            // Add this variable to the list of already assessed
                            // points
                            already_assessed.insert(point);

                            point
                        })
                        .collect();

                    // Add the merge to the change list
                    changes.push(SimulationStateChange::Merge {
                        variable_points: variables,
                        new_position: next_position,
                    });
                } else {
                    // If not, the variable should stay put
                    changes.push(SimulationStateChange::DoNothing);
                }
            } else {
                // If there is only one way to get to this place, move the
                // variable

                // Check if the next point is the exit
                if next_position == self.program.exit {
                    // If so, kill this variable
                    changes.push(SimulationStateChange::Kill {
                        variable_point: *point,
                    });
                // Otherwise, move it
                } else {
                    changes.push(SimulationStateChange::Move {
                        variable_point: *point,
                        new_position: next_position,
                    });
                }
            }
        });

        // Iterate over everything in the changes list
        for change in changes {
            match change {
                SimulationStateChange::Move {
                    variable_point,
                    new_position,
                } => {
                    // Add the variable to the new position
                    self.variables
                        .insert(new_position, *self.variables.get(&variable_point).unwrap());

                    // Remove the variable from the old position
                    self.variables.remove(&variable_point);
                }
                SimulationStateChange::Merge {
                    variable_points: variables,
                    new_position,
                } => {
                    // Merge the variables into a new variable, and complete the
                    // operation at this location
                    let values = variables
                        .iter()
                        .map(|point| self.variables.get(point).unwrap().value)
                        .collect::<Vec<i32>>();

                    let result: i32 = match self.program.file.get(&new_position).unwrap().syntax {
                        Syntax::Add => values.iter().sum(),
                        Syntax::Subtract => todo!(),
                        Syntax::Multiply => values.iter().product(),
                        Syntax::Divide => todo!(),
                        Syntax::Modulo => todo!(),
                        Syntax::Max => todo!(),
                        Syntax::Min => todo!(),
                        Syntax::GreaterThan => todo!(),
                        Syntax::LessThan => todo!(),
                        Syntax::Equal => todo!(),
                        _ => unreachable!(),
                    };

                    // Remove the old variables from the hashmap
                    variables.iter().for_each(|point| {
                        self.variables.remove(point);
                    });

                    // Create the new variable and add it to the hashmap
                    self.variables.insert(
                        new_position,
                        Variable {
                            value: result,
                            spawner: None,
                        },
                    );
                }
                SimulationStateChange::Spawn { spawner } => {
                    // Spawn a new variable at the spawner
                    let new_variable = Variable {
                        value: self.program.spawners.get_mut(&spawner).unwrap().spawn(),
                        spawner: Some(spawner),
                    };

                    self.variables.insert(spawner, new_variable);
                }
                SimulationStateChange::DoNothing => {}
                SimulationStateChange::Kill { variable_point } => {
                    // Print the variable's value
                    // println!(
                    //     "Output: {}",
                    //     self.variables.get(&variable_point).unwrap().value
                    // );

                    self.outputs.push(format!(
                        "{}",
                        self.variables.get(&variable_point).unwrap().value
                    ));

                    // Kill the variable
                    self.variables.remove(&variable_point);
                }
            }
        }
    }
}
