#[derive(Clone)]
pub struct Memory {
    pub values: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        let values: Vec<u8> = vec![0; size];
        return Memory { values };
    }

    pub fn from_values(values: Vec<u8>) -> Memory {
        return Memory { values };
    }

    pub fn read(&self, index: usize) -> Option<u8> {
        if index >= self.values.len() {
            return None;
        }
        return Some(self.values[index]);
    }

    pub fn write(&mut self, index: usize, value: u8) -> bool {
        if index >= self.values.len() {
            return false;
        }
        self.values[index] = value;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_in_bounds() {
        let mut memory = Memory::new(10);
        assert_eq!(memory.write(0, 10), true);
        assert_eq!(memory.values[0], 10);
    }

    #[test]
    fn test_write_out_of_bounds() {
        let mut memory = Memory::new(10);
        assert_eq!(memory.write(100, 10), false);
    }

    #[test]
    fn test_read_in_bounds() {
        let mut memory = Memory::new(10);
        assert_eq!(memory.write(0, 10), true);
        assert_eq!(memory.read(0), Some(10));
    }

    #[test]
    fn test_read_out_of_bounds() {
        let mut memory = Memory::new(10);
        assert_eq!(memory.write(0, 10), true);
        assert_eq!(memory.read(100), None);
    }
}
