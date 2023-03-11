# bf-interpreter-cli

BF Interpreter CLI written in Rust

This is a small CLI interpreter for [Brain****](https://en.wikipedia.org/wiki/Brainfuck) esoteric language.

See the [language page](https://tinyurl.com/bf-page) for language implementation.

# Usage 

```cli
CLI interpreter for Brain**** language

Usage: bf_interpreter.exe [OPTIONS] <FILE_NAME>

Arguments:
  <FILE_NAME>  File to interpret

Options:
  -i, --input <INPUT>              Input data to pass to the program [default: ]
  -s, --sanitize-input             Whether the input will be sanitized (new line and carrier return removed). [default: true]
  -n, --interactive                Whether to ask for user input during execution. Always be false if input passed. [default: false]
  -e, --eof <EOF>                  What character to be returned as EOF when there is no more input to be read [default: ]
  -c, --cell-size <CELL_SIZE>      Size of the memory cell in bits, allowed values: 8, 16, 32 [default: 8]
  -m, --memory-size <MEMORY_SIZE>  Size of the memory [default: 30000]
  -h, --help                       Print help
  -V, --version                    Print version
  ```

# License

This program has been released under [Apache 2.0 License](LICENSE).
