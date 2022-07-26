use moveslice::Moveslice;
use rand::rngs::SmallRng;

use crate::direction::Direction;
use crate::instruction::Instruction;
use crate::memory::Memory;
use serde_big_array::BigArray;
use serde_derive::{Deserialize, Serialize};

const STACK_SIZE: usize = 64;
const ADDRESS_DISTANCE: usize = 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Processor {
    pub ip: usize,
    stack_pointer: usize,
    jumped: bool,
    pub alive: bool,
    pub want_start: Option<usize>,
    pub want_split: Option<(Direction, usize)>,
    pub want_merge: Option<Direction>,
    pub want_eat: bool,
    pub want_grow: bool,
    #[serde(with = "BigArray")]
    stack: [u64; STACK_SIZE],
}

impl Processor {
    pub fn new(ip: usize) -> Processor {
        return Processor {
            ip,
            stack: [0; STACK_SIZE],
            jumped: false,
            alive: true,
            want_start: None,
            want_split: None,
            want_merge: None,
            want_eat: false,
            want_grow: false,
            stack_pointer: 0,
        };
    }

    pub fn current_stack(&self) -> &[u64] {
        &self.stack[0..self.stack_pointer]
    }

    pub fn execute(&mut self, memory: &mut Memory, rng: &mut SmallRng) -> bool {
        if !self.alive {
            return false;
        }
        if self.ip >= memory.values.len() {
            self.alive = false;
            return false;
        }
        let value = memory.values[self.ip];
        let instruction: Option<Instruction> = Instruction::decode(value);
        match instruction {
            Some(instruction) => instruction.execute(self, memory, rng),
            None => {
                // no op, we cannot interpret this as a valid instruction
            }
        }
        if !self.jumped {
            self.ip += 1;
        } else {
            self.jumped = false;
        }
        return true;
    }

    pub fn execute_amount(
        &mut self,
        memory: &mut Memory,
        rng: &mut SmallRng,
        amount: usize,
    ) -> usize {
        self.want_start = None;
        self.want_eat = false;
        self.want_grow = false;
        self.want_split = None;
        self.want_merge = None;
        let mut total = 0;
        for _ in 0..amount {
            if self.execute(memory, rng) {
                total += 1;
            }
        }
        return total;
    }

    pub fn start(&mut self, address: usize) {
        self.want_start = Some(address);
    }

    pub fn end(&mut self) {
        self.alive = false;
    }

    pub fn jump(&mut self, address: usize) {
        self.ip = address;
        self.jumped = true;
    }

    pub fn call(&mut self, address: usize) {
        self.push(self.ip as u64);
        self.jump(address);
    }

    pub fn address(&self) -> u64 {
        self.ip as u64
    }

    pub fn push(&mut self, value: u64) {
        if self.stack_pointer >= STACK_SIZE {
            self.compact_stack();
        }
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    fn compact_stack(&mut self) {
        self.stack_pointer = STACK_SIZE / 2;
        self.stack.moveslice(usize::from(self.stack_pointer).., 0);
    }

    pub fn dup(&mut self) {
        if self.stack_pointer < 1 {
            return;
        }
        self.push(self.top());
    }

    pub fn dup2(&mut self) {
        if self.stack_pointer < 2 {
            return;
        }
        let first = self.stack[self.stack_pointer - 2];
        let second = self.stack[self.stack_pointer - 1];
        self.push(first);
        self.push(second);
    }

    pub fn pop(&mut self) -> u64 {
        if self.stack_pointer == 0 {
            return u64::MAX;
        }
        self.stack_pointer -= 1;
        return self.stack[self.stack_pointer];
    }

    pub fn pop_address(&mut self, memory: &Memory) -> Option<usize> {
        if self.stack_pointer == 0 {
            return None;
        }
        self.stack_pointer -= 1;
        let result = self.stack[self.stack_pointer] as usize;
        if result >= memory.values.len() {
            return None;
        }
        let distance = if result > self.ip {
            result - self.ip
        } else {
            self.ip - result
        };
        if distance > ADDRESS_DISTANCE {
            return None;
        }
        return Some(result);
    }

    fn top(&self) -> u64 {
        self.stack[self.stack_pointer - 1]
    }

    pub fn drop(&mut self) {
        if self.stack_pointer == 0 {
            return;
        }
        self.stack_pointer -= 1;
    }

    pub fn swap(&mut self) {
        if self.stack_pointer < 2 {
            return;
        }
        let under = self.stack_pointer - 2;
        let over = self.stack_pointer - 1;
        let temp = self.stack[over];
        self.stack[over] = self.stack[under];
        self.stack[under] = temp;
    }

    pub fn over(&mut self) {
        if self.stack_pointer < 2 {
            return;
        }
        let under = self.stack_pointer - 2;
        self.push(self.stack[under]);
    }

    pub fn rot(&mut self) {
        if self.stack_pointer < 3 {
            return;
        }
        let one = self.stack_pointer - 3;
        let two = self.stack_pointer - 2;
        let three = self.stack_pointer - 1;
        let temp = self.stack[one];
        self.stack[one] = self.stack[two];
        self.stack[two] = self.stack[three];
        self.stack[three] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_stack() {
        let mut processor = Processor::new(0);
        let stack_size: u64 = STACK_SIZE.try_into().unwrap();
        for value in 0..stack_size {
            processor.push(value);
        }
        assert_eq!(processor.stack_pointer, STACK_SIZE);
        assert_eq!(processor.top(), stack_size - 1);

        // push one more item which should cause stack compaction
        processor.push(100);

        assert_eq!(processor.stack_pointer, STACK_SIZE / 2 + 1);
        assert_eq!(processor.top(), 100);
    }

    #[test]
    fn test_pop() {
        let mut processor = Processor::new(0);
        processor.push(10);
        processor.push(100);
        assert_eq!(processor.pop(), 100);
        assert_eq!(processor.pop(), 10);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut processor = Processor::new(0);
        processor.push(10);
        assert_eq!(processor.pop(), 10);
        assert_eq!(processor.pop(), u64::MAX);
    }

    #[test]
    fn test_pop_address() {
        let mut memory = Memory::new(100);
        let mut processor = Processor::new(0);
        processor.push(10);
        assert_eq!(processor.pop_address(&mut memory), Some(10));
        assert_eq!(processor.pop_address(&mut memory), None);
    }

    #[test]
    fn test_pop_address_out_of_bounds_of_memory() {
        let mut memory = Memory::new(100);
        let mut processor = Processor::new(0);
        processor.push(1000);
        assert_eq!(processor.pop_address(&mut memory), None);
    }

    #[test]
    fn test_pop_address_beyond_address_distance() {
        let mut memory = Memory::new(ADDRESS_DISTANCE * 10);
        let mut processor = Processor::new(0);
        let address_distance: u64 = ADDRESS_DISTANCE.try_into().unwrap();
        processor.push(address_distance + 1); // cannot address this
        assert_eq!(processor.pop_address(&mut memory), None);
    }

    #[test]
    fn test_pop_address_beyond_address_distance_other_direction() {
        let mut memory = Memory::new(ADDRESS_DISTANCE * 10);
        let mut processor = Processor::new(ADDRESS_DISTANCE * 2);
        processor.push(0); // cannot address this
        assert_eq!(processor.pop_address(&mut memory), None);
    }

    #[test]
    fn test_drop() {
        let mut processor = Processor::new(0);
        processor.push(10);
        processor.push(100);
        processor.drop();
        assert_eq!(processor.pop(), 10);
    }

    #[test]
    fn test_swap() {
        let mut processor = Processor::new(0);
        processor.push(1);
        processor.push(2);
        processor.swap();
        assert_eq!(processor.pop(), 1);
        assert_eq!(processor.pop(), 2);
    }

    #[test]
    fn test_swap_not_enough_on_stack() {
        let mut processor = Processor::new(0);
        processor.push(1);
        processor.swap();
        assert_eq!(processor.pop(), 1);
    }

    #[test]
    fn test_over() {
        let mut processor = Processor::new(0);
        processor.push(1);
        processor.push(2);
        processor.over();
        assert_eq!(processor.pop(), 1);
        assert_eq!(processor.pop(), 2);
        assert_eq!(processor.pop(), 1);
    }

    #[test]
    fn test_over_not_enough_on_stack() {
        let mut processor = Processor::new(0);
        processor.push(1);
        processor.over();
        assert_eq!(processor.pop(), 1);
        assert_eq!(processor.pop(), u64::MAX);
    }

    #[test]
    fn test_rot() {
        let mut processor = Processor::new(0);
        processor.push(1);
        processor.push(2);
        processor.push(3);
        processor.rot();
        assert_eq!(processor.pop(), 1);
        assert_eq!(processor.pop(), 3);
        assert_eq!(processor.pop(), 2);
    }
}
