use crate::interpreter::BfMemory;

pub struct Memory<T: MemoryCell + Clone> {
    memory: Vec<T>,
    current_cell: usize,
}

pub trait MemoryCell {
    fn increase(&mut self);
    fn decrease(&mut self);
    fn write_char(&mut self, input: char);
    fn read_char(&self) -> char;
    fn is_empty(&self) -> bool;
}

impl<T: MemoryCell + Clone> Memory<T> {
    pub fn new(length: usize, initialize_with: T) -> Memory<T> {
        Self {
            memory: vec![initialize_with; length],
            current_cell: 0,
        }
    }
}

impl<T: MemoryCell + Clone> BfMemory for Memory<T> {
    fn next(&mut self) {
        self.current_cell = std::cmp::min(self.current_cell + 1, self.memory.len() - 1);
    }

    fn prev(&mut self) {
        self.current_cell = self.current_cell.checked_sub(1).unwrap_or(0);
    }

    fn is_cell_zero_value(&self) -> bool {
        self.memory[self.current_cell].is_empty()
    }

    fn increase_cell(&mut self) {
        self.memory[self.current_cell].increase();
    }

    fn decrease_cell(&mut self) {
        self.memory[self.current_cell].decrease();
    }

    fn read_char(&self) -> char {
        self.memory[self.current_cell].read_char()
    }

    fn write_char(&mut self, input: char) {
        self.memory[self.current_cell].write_char(input);
    }
}

macro_rules! implement_memory_cell_type {
    ($type: ident) => {
        impl MemoryCell for $type {
            fn increase(&mut self) {
                *self = self.wrapping_add(1);
            }

            fn decrease(&mut self) {
                *self = self.wrapping_sub(1);
            }

            fn read_char(&self) -> char {
                (*self as u8) as char
            }

            fn write_char(&mut self, input: char) {
                *self = input as $type;
            }

            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
    };
}

implement_memory_cell_type!(u8);
implement_memory_cell_type!(u16);
implement_memory_cell_type!(u32);
