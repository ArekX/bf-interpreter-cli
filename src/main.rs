use std::{
    fs::File,
    io::{stdin, Read},
    process::ExitCode,
};

use clap::Parser;
use input::InputSource;
use interpreter::{interpret_code, BfInput};
use memory::{Memory, MemoryCell};

mod input;
mod interpreter;
mod memory;

/// CLI interpreter for Brain**** language
#[derive(Parser, Debug)]
#[command(
    name = "BF interpreter", 
    author = "Aleksandar Panic", 
    version,
    long_about = None
)]
struct Args {
    /// File to interpret
    file_name: String,

    /// Input data to pass to the program
    #[arg(short, long, default_value_t = String::from(""))]
    input: String,

    /// Whether the input will be sanitized (new line and carrier return removed). [default: true]
    #[arg(short, long, default_value_t = true)]
    sanitize_input: bool,

    /// Whether to ask for user input during execution. Always be false if input passed. [default: false]
    #[arg(short = 'n', long, default_value_t = false)]
    interactive: bool,

    /// What character to be returned as EOF when there is no more input to be read.
    #[arg(short, long, default_value_t = 0 as char)]
    eof: char,

    /// Size of the memory cell in bits, allowed values: 8, 16, 32
    #[arg(short, long, default_value_t = 8)]
    cell_size: u8,

    /// Size of the memory.
    #[arg(short, long, default_value_t = 30000)]
    memory_size: usize,
}

fn execute_code<CellType: MemoryCell + Clone>(
    memory_length: usize,
    initial_memory_value: CellType,
    code: &String,
    input: &mut impl BfInput,
) {
    let mut memory = Memory::<CellType>::new(memory_length, initial_memory_value);
    interpret_code(&code, &mut memory, input);
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut input_file = File::open(&args.file_name).expect("Could not open an input file.");

    let mut code = String::new();
    input_file
        .read_to_string(&mut code)
        .expect("Could not read from input file.");

    let mut input_source = args.input;

    if input_source.len() == 0 && !atty::is(atty::Stream::Stdin) {
        stdin().lock().read_to_string(&mut input_source).unwrap();
    }

    let mut source = InputSource::from(
        &input_source,
        args.eof,
        args.interactive,
        args.sanitize_input,
    );

    match args.cell_size {
        8 => execute_code(args.memory_size, 0 as u8, &code, &mut source),
        16 => execute_code(args.memory_size, 0 as u16, &code, &mut source),
        32 => execute_code(args.memory_size, 0 as u32, &code, &mut source),
        _ => println!("Memory size must be one of: 8, 16, 32"),
    }

    ExitCode::SUCCESS
}
