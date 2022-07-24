use std::collections::HashMap;

use crate::prelude::*;

pub type FileMap = HashMap<Point, Place>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub file: FileMap,
    pub spawners: HashMap<Point, Spawner>,
    pub exit: Point,
    pub bounds: Bounds,
    pub folder: String,
}

impl Program {
    pub fn new(file: FileMap, bounds: Bounds, folder: String) -> Program {
        Program {
            file,
            spawners: HashMap::new(),
            exit: Point { x: 0, y: 0 },
            bounds,
            folder,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl Bounds {
    pub fn new() -> Self {
        Self {
            min_x: std::i32::MAX,
            max_x: std::i32::MIN,
            min_y: std::i32::MAX,
            max_y: std::i32::MIN,
        }
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.min_x = std::cmp::min(self.min_x, x);
        self.max_x = std::cmp::max(self.max_x, x);
        self.min_y = std::cmp::min(self.min_y, y);
        self.max_y = std::cmp::max(self.max_y, y);
    }
}
