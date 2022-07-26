use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Place {
    pub next: Option<Point>,
    pub prev: Vec<Option<Point>>,
    pub syntax: Syntax,
}

impl Place {
    pub fn new(syntax: Syntax) -> Self {
        Self {
            next: None,
            prev: Vec::new(),
            syntax,
        }
    }
}
