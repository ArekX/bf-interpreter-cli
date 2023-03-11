use std::io::BufRead;

use crate::interpreter::BfInput;

pub struct InputSource {
    buffer: String,
    eof_character: char,
    is_interactive: bool,
    should_sanitize: bool,
}

impl InputSource {
    pub fn from(input: &String, eof: char, is_interactive: bool, should_sanitize: bool) -> Self {
        let mut instance = Self {
            buffer: input.clone(),
            eof_character: eof,
            is_interactive,
            should_sanitize,
        };

        instance.sanitize_buffer();

        instance
    }

    fn sanitize_buffer(&mut self) {
        if !self.should_sanitize {
            return;
        }

        while self.buffer.len() > 0 && (self.buffer.ends_with("\n") || self.buffer.ends_with("\r"))
        {
            self.buffer.pop();
        }
    }
}

impl BfInput for InputSource {
    fn read(&mut self) -> char {
        if self.buffer.len() > 0 {
            return self.buffer.remove(0);
        }

        if self.is_interactive {
            std::io::stdin().lock().read_line(&mut self.buffer).unwrap();
            self.sanitize_buffer();
            if self.buffer.len() > 0 {
                return self.buffer.remove(0);
            }
        }

        return self.eof_character;
    }
}
