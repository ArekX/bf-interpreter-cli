use std::io::Write;

pub enum Instruction {
    CellNext,
    CellPrev,
    LoopStart,
    LoopEnd,
    Inc,
    Dec,
    Output,
    Input,
}

pub trait BfInput {
    fn read(&mut self) -> char;
}

pub trait BfMemory {
    fn next(&mut self);
    fn prev(&mut self);
    fn increase_cell(&mut self);
    fn decrease_cell(&mut self);

    fn is_cell_zero_value(&self) -> bool;
    fn read_char(&self) -> char;
    fn write_char(&mut self, input: char);
}

fn parse(code: &String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for char in code.chars() {
        match char {
            '>' => instructions.push(Instruction::CellNext),
            '<' => instructions.push(Instruction::CellPrev),
            '[' => instructions.push(Instruction::LoopStart),
            ']' => instructions.push(Instruction::LoopEnd),
            '+' => instructions.push(Instruction::Inc),
            '-' => instructions.push(Instruction::Dec),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            _ => {}
        }
    }

    instructions
}

pub fn interpret_code(code: &String, memory: &mut impl BfMemory, input: &mut impl BfInput) {
    let mut pc: usize = 0;
    let mut pc_stack: Vec<usize> = vec![];
    let instructions = parse(&code);

    while pc < instructions.len() {
        if let Some(instruction) = instructions.get(pc) {
            match instruction {
                Instruction::CellNext => memory.next(),
                Instruction::CellPrev => memory.prev(),
                Instruction::LoopStart => {
                    if memory.is_cell_zero_value() {
                        pc = find_matching_loop_end(&instructions, pc);
                    } else {
                        pc_stack.push(pc.checked_sub(1).unwrap_or(0));
                    }
                }
                Instruction::LoopEnd => {
                    if let Some(new_pc) = pc_stack.pop() {
                        if !memory.is_cell_zero_value() {
                            pc = new_pc;
                        }
                    }
                }
                Instruction::Inc => memory.increase_cell(),
                Instruction::Dec => memory.decrease_cell(),
                Instruction::Output => {
                    print!("{}", memory.read_char());
                    std::io::stdout().flush().unwrap();
                }
                Instruction::Input => memory.write_char(input.read()),
            }
        }

        pc += 1;
    }
}

fn find_matching_loop_end(instructions: &Vec<Instruction>, from_pc: usize) -> usize {
    let mut pc = from_pc;
    let mut bracket_count = 0;

    while pc < instructions.len() {
        match instructions[pc] {
            Instruction::LoopStart => bracket_count += 1,
            Instruction::LoopEnd => bracket_count -= 1,
            _ => {}
        }

        if bracket_count == 0 {
            return pc;
        }

        pc += 1;
    }

    instructions.len() - 1
}
