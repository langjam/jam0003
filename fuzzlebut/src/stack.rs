use std::{
    ops::{
        Index,
        IndexMut,
    },
    fmt::Debug,
};


/// Basically a thin wrapper around a Vec<T>, but with a different indexing method: from the top of
/// the stack.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Stack<T> {
    items:Vec<T>,
}
impl<T> Stack<T> {
    /// Create a new stack
    pub fn new()->Self {
        Stack {
            items:Vec::new(),
        }
    }
    /// Remove the top item
    pub fn pop(&mut self)->Option<T> {
        return self.items.pop();
    }
    /// Push an item
    pub fn push(&mut self,item:T) {
        self.items.push(item);
    }
    /// Rotates the top item to the bottom
    pub fn rotate(&mut self) {
        self.items.rotate_right(1);
    }
    /// Rotates the bottom item to the top
    pub fn rotate_rev(&mut self) {
        self.items.rotate_left(1);
    }
    #[inline]
    pub fn len(&self)->usize {self.items.len()}
}
impl<T:Debug> Stack<T> {
    /// Iterate through the items on the stack and print them out from top to bottom where top is
    /// `0` and bottom is `-len`
    pub fn debug_print(&self) {
        println!("Stack:");
        for (i,item) in self.items.iter().rev().enumerate() {
            if i==0 {
                print!("0     ");
            } else {
                print!("-{:<4} ",i);
            }
            println!("{:?}",item);
        }
    }
}
impl<T> Index<usize> for Stack<T> {
    type Output=T;
    fn index(&self,index:usize)->&T {
        let last=self.items.len()-1;
        if last<index {
            panic!("Stack index {} is out of range",index);
        }
        return &self.items[last-index];
    }
}
impl<T> IndexMut<usize> for Stack<T> {
    fn index_mut(&mut self,index:usize)->&mut T {
        let last=self.items.len()-1;
        if last<index {
            panic!("Stack index {} is out of range",index);
        }
        return &mut self.items[last-index];
    }
}
