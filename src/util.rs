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

#[macro_export]
macro_rules! component {
    ($id:ident { $($field_id:ident: $field_ty:ty),* }) => {
        struct $id {
            $($field_id: $field_ty),*
        }


        impl $id {
            pub fn new($($field_id: $field_ty),*) -> Self {
                Self { $($field_id),* }
            }
        }
    };

    ($($id:ident { $($field_id:ident: $field_ty:ty),* }),*) => {
        $(component!($id { $($field_id: $field_ty),* });)*
    };
}

#[macro_export]
macro_rules! global_pipe {
    ($id:ident, $ty:ty) => {
        lazy_static! {
            pub static ref $id: Arc<Mutex<Pipe<$ty>>> = Arc::new(Mutex::new(Pipe::new()));
        }
    };
}

#[macro_export]
macro_rules! local_pipe {
    ($id:ident, $ty:ty) => {
        let mut $id: Pipe<$ty> = Pipe::new();
    };
}

#[macro_export]
macro_rules! machine {
    ($id:ident = |$($param:ident: $ty:ty),*| -> $output:ty $proc:block) => {
        fn $id($($param: $ty),*) -> $output {
            $proc
        }
    };

    ($($id:ident = |$($param:ident: $ty:ty),*| -> $output:ty $proc:block),*) => {
        $(machine!($id  = |$($param: $ty),*| -> $output $proc );)*
    };
}

#[macro_export]
macro_rules! void_machine {
    ($id:ident = |$($param:ident: $ty:ty),*|$proc:block) => {
        fn $id($($param: $ty),*) {
            $proc
        }
    };

    ($($id:ident = |$($param:ident: $ty:ty),*| $proc:block),*) => {
        $(void_machine!($id  = |$($param: $ty),*| $proc );)*
    };
}

#[cfg(test)]
mod test {
    use super::*;

    global_pipe!(PIPE_1, u8);
    component![
        Vec2U8 {
            field_1: u8,
            field_2: u8
        },
        Vec2U16 {
            field_1: u16,
            field_2: u16
        }
    ];
    machine![
        add = |x: u8, y: u8| -> u8 { x + y },
        sub = |x: u8, y: u8| -> u8 { x - y },
        recieve = |pipe: &mut Pipe<u8>| -> Option<u8> { pipe.read() }
    ];
    void_machine![
        send = |x: u8, pipe: &mut Pipe<u8>| {
            pipe.write(x);
        }
    ];

    #[test]
    fn components_and_pipes_work() {
        local_pipe!(pipe_2, u8);

        let numbers = Vec2U8::new(1, 1);

        let result = add(numbers.field_1, numbers.field_2);
        send(result, &mut pipe_2);

        let result = sub(numbers.field_1, numbers.field_2);
        send(result, &mut pipe_2);

        let result = recieve(&mut pipe_2);
        println!("{result:?}");

        let result = recieve(&mut pipe_2);
        println!("{result:?}");
    }
}
