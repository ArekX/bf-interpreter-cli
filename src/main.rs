use std::{env, fs::File, io::Read, process::ExitCode};

use interpreter::{interpret_code, BfInput};
use memory::{Memory, MemoryCell};

mod interpreter;
mod memory;

impl BfInput for String {
    fn read(&mut self) -> char {
        if self.len() == 0 {
            return 0 as char;
        }

        self.remove(0)
    }
}

fn execute_code<CellType: MemoryCell + Clone>(
    memory_length: usize,
    initial_memory_value: CellType,
    code: &String,
    input: &String,
) {
    let mut memory = Memory::<CellType>::new(memory_length, initial_memory_value);
    let mut source = input.clone();

    interpret_code(&code, &mut memory, &mut source);
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an input file.");
        return ExitCode::FAILURE;
    }

    let mut input_file = File::open(&args[1]).expect("Could not open an input file.");

    let mut code = String::new();
    input_file
        .read_to_string(&mut code)
        .expect("Could not read from input file.");

    execute_code(30000, 0 as u8, &code, &"opop".to_owned());

    ExitCode::SUCCESS
}
