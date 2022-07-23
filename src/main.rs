#![allow(unused)]

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub struct Pipe<T>(Vec<T>);

impl<T> Pipe<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn read(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn write(&mut self, value: T) {
        self.0.push(value);
    }
}

macro_rules! component {
    ($id:ident, $($field_id:ident: $field_ty:ty),*) => {
        struct $id {
            $($field_id: $field_ty),*
        }


        impl $id {
            pub fn new($($field_id: $field_ty),*) -> Self {
                Self { $($field_id),* }
            }
        }
    }
}

macro_rules! global_pipe {
    ($id:ident, $ty:ty) => {
        lazy_static! {
            pub static ref $id: Arc<Mutex<Pipe<$ty>>> = Arc::new(Mutex::new(Pipe::new()));
        }
    };
}

macro_rules! local_pipe {
    ($id:ident, $ty:ty) => {
        let mut $id: Pipe<$ty> = Pipe::new();
    };
}

macro_rules! machine {
    ($id:ident, $($param:ident: $ty:ty),* => $proc:block) => {
        fn $id($($param: $ty),*) {
            $proc
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    global_pipe!(PIPE_1, u8);
    component!(Temp, field_1: u8, field_2: u8);
    machine!(add, x: u8, y: u8, output: &mut Pipe<u8> => {
        output.write(x + y);
    });

    #[test]
    fn components_and_pipes_work() {
        local_pipe!(pipe_2, u8);

        let temp = Temp::new(1, 2);

        add(temp.field_1, temp.field_2, &mut pipe_2);

        println!("{:?}", pipe_2.read());
    }
}
