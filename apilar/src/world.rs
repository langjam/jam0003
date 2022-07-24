use crate::computer::Computer;
use crate::direction::Direction;
use rand::rngs::SmallRng;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub resources: u64,
    pub computer: Option<Computer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub width: usize,
    pub height: usize,
    eat_amount: u64,
    pub rows: Vec<Vec<Location>>,
}

type Coords = (usize, usize);

impl World {
    pub fn new(width: usize, height: usize, eat_amount: u64, resources: u64) -> World {
        let mut rows: Vec<Vec<Location>> = Vec::new();
        for _ in 0..height {
            let mut column_vec: Vec<Location> = Vec::new();
            for _ in 0..width {
                column_vec.push(Location::new(resources));
            }
            rows.push(column_vec);
        }
        World {
            width,
            eat_amount,
            height,
            rows,
        }
    }

    fn neighbor_coords(&self, coords: Coords, direction: Direction) -> Coords {
        let (x, y) = coords;
        let ix = x as i32;
        let iy = y as i32;
        let (nx, ny): (i32, i32) = match direction {
            Direction::North => (ix, iy - 1),
            Direction::East => (ix + 1, iy),
            Direction::South => (ix, iy + 1),
            Direction::West => (ix - 1, iy),
        };
        let rx = nx.rem_euclid(self.width as i32);
        let ry = ny.rem_euclid(self.height as i32);
        return (rx as usize, ry as usize);
    }

    pub fn set(&mut self, (x, y): Coords, computer: Computer) {
        self.rows[y][x].computer = Some(computer);
    }

    pub fn get(&self, (x, y): Coords) -> &Location {
        &self.rows[y][x]
    }

    pub fn get_mut<'a>(&'a mut self, (x, y): Coords) -> &'a mut Location {
        &mut self.rows[y][x]
    }

    pub fn get_random_coords(&self, rng: &mut SmallRng) -> Coords {
        let x = rng.gen_range(0..self.width);
        let y = rng.gen_range(0..self.height);
        (x, y)
    }

    pub fn is_empty(&self, coords: Coords) -> bool {
        match &self.get(coords).computer {
            Some(_) => false,
            None => true,
        }
    }

    pub fn update(&mut self, rng: &mut SmallRng, amount_per_processor: usize) {
        let coords = self.get_random_coords(rng);

        let location = self.get_mut(coords);
        location.update(rng, amount_per_processor);

        if let Some((neighbor_coords, address)) = self.want_split(coords) {
            self.split(coords, neighbor_coords, address);
        }

        if let Some(neighbor_coords) = self.want_merge(coords) {
            let neighbor_computer = self.get(neighbor_coords).computer.clone();
            if let Some(neighbor_computer) = neighbor_computer {
                self.merge(coords, neighbor_coords, &neighbor_computer);
            }
        }

        if self.want_eat(coords) {
            self.eat(coords);
        }
    }

    pub fn mutate(&mut self, rng: &mut SmallRng, amount_memory: u64, amount_processors: u64) {
        let coords = self.get_random_coords(rng);
        let location = self.get_mut(coords);
        if let Some(computer) = &mut location.computer {
            computer.mutate_memory(rng, amount_memory);
            computer.mutate_processors(rng, amount_processors);
        }
    }

    fn want_split(&self, coords: Coords) -> Option<(Coords, usize)> {
        if let Some(computer) = &self.get(coords).computer {
            if let Some((direction, address)) = computer.want_split() {
                let neighbor_coords = self.neighbor_coords(coords, direction);
                if self.is_empty(neighbor_coords) {
                    return Some((neighbor_coords, address));
                }
            }
        }
        return None;
    }

    fn want_merge(&self, coords: Coords) -> Option<Coords> {
        if let Some(computer) = &self.get(coords).computer {
            if let Some(direction) = computer.want_merge() {
                let neighbor_coords = self.neighbor_coords(coords, direction);
                if !self.is_empty(neighbor_coords) {
                    return Some(neighbor_coords);
                }
            }
        }
        return None;
    }

    fn want_eat(&self, coords: Coords) -> bool {
        if let Some(computer) = &self.get(coords).computer {
            return computer.want_eat();
        }
        return false;
    }

    fn split(&mut self, coords: Coords, neighbor_coords: Coords, address: usize) {
        let computer = &mut self.get_mut(coords).computer;
        let splitted: Option<Computer> = if let Some(computer) = computer {
            Some(computer.split(address))
        } else {
            None
        };
        let neighbor_location = self.get_mut(neighbor_coords);
        neighbor_location.computer = splitted;
    }

    fn merge(&mut self, coords: Coords, neighbor_coords: Coords, neighbor_computer: &Computer) {
        let computer = &mut self.get_mut(coords).computer;
        if let Some(computer) = computer {
            computer.merge(neighbor_computer);
        }
        let neighbor_location = self.get_mut(neighbor_coords);
        neighbor_location.computer = None;
    }

    fn eat(&mut self, coords: Coords) {
        let eat_amount = self.eat_amount;
        let location = self.get_mut(coords);
        if let Some(computer) = &mut location.computer {
            let amount = if location.resources >= eat_amount {
                eat_amount
            } else {
                location.resources
            };

            computer.resources += amount;
            location.resources -= amount;
        }
    }

    pub fn computers_amount(&self) -> u64 {
        let mut total = 0;
        for row in &self.rows {
            for location in row {
                if location.computer.is_some() {
                    total += 1;
                }
            }
        }
        return total;
    }

    pub fn processors_amount(&self) -> u64 {
        let mut total = 0;
        for row in &self.rows {
            for location in row {
                if let Some(computer) = &location.computer {
                    total += computer.processors.len() as u64;
                }
            }
        }
        return total;
    }
}

impl Location {
    pub fn new(resources: u64) -> Location {
        Location {
            resources,
            computer: None,
        }
    }

    pub fn update(&mut self, rng: &mut SmallRng, amount_per_processor: usize) {
        let mut eliminate_computer: bool = false;

        if let Some(computer) = &mut self.computer {
            if computer.processors.len() == 0 {
                self.resources += computer.resources + computer.memory.values.len() as u64;
                eliminate_computer = true;
            } else {
                computer.execute(rng, amount_per_processor);
            }
        }
        if eliminate_computer {
            self.computer = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbor_out_of_bounds() {
        let world = World::new(5, 5, 10, 5);
        assert_eq!(world.neighbor_coords((2, 2), Direction::North), (2, 1));
        assert_eq!(world.neighbor_coords((2, 2), Direction::South), (2, 3));
        assert_eq!(world.neighbor_coords((2, 2), Direction::West), (1, 2));
        assert_eq!(world.neighbor_coords((2, 2), Direction::East), (3, 2));

        assert_eq!(world.neighbor_coords((1, 0), Direction::North), (1, 4));
        assert_eq!(world.neighbor_coords((1, 4), Direction::South), (1, 0));
        assert_eq!(world.neighbor_coords((0, 2), Direction::West), (4, 2));
        assert_eq!(world.neighbor_coords((4, 2), Direction::East), (0, 2));
    }
}
