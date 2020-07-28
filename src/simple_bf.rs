use std::io::Read;

// Total size of brainfuck memory
const BRAINFUCK_MEMORY_SIZE: usize = 4096;

pub struct BrainfuckCallStack {
    calls: Vec<usize>,
}

impl BrainfuckCallStack {
    pub fn new() -> BrainfuckCallStack {
        BrainfuckCallStack { calls: Vec::new() }
    }

    pub fn push(&mut self, value: usize) {
        self.calls.push(value);
    }

    pub fn top(&self) -> Option<usize> {
        let top_pt = self.calls.last();

        if top_pt.is_some() {
            Some(*top_pt.unwrap())
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        let top = self.top();

        if top.is_some() {
            self.calls.pop();
        }

        top
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BrainfuckToken {
    IncMemoryPointer, // >
    DecMemoryPointer, // <
    IncMemory,        // +
    DecMemory,        // -
    Output,           // .
    Input,            // ,
    LoopStart,        // [
    LoopEnd,          // ]
}

pub struct BrainfuckMachine {
    callstack: BrainfuckCallStack,
    program: Vec<BrainfuckToken>,
    memory: [u8; BRAINFUCK_MEMORY_SIZE],
    memory_pointer: usize,
    program_counter: usize,
}

impl BrainfuckMachine {
    pub fn new() -> BrainfuckMachine {
        BrainfuckMachine {
            callstack: BrainfuckCallStack::new(),
            program: Vec::new(),
            memory: [0; BRAINFUCK_MEMORY_SIZE],
            memory_pointer: 0,
            program_counter: 0,
        }
    }

    pub fn load_program(&mut self, source: &str) {
        // Clear initial program memory
        self.program.clear();

        // Loop through source
        for c in source.chars() {
            let token = BrainfuckMachine::tokenise(c);

            if token.is_some() {
                self.program.push(token.unwrap());
            }
        }

        println!("Loaded {} tokens.", self.program.len());
    }

    pub fn run(&mut self) {
        self.program_counter = 0;
        self.memory_pointer = 0;

        while self.program_counter < self.program.len() {
            // Get next token
            let token = self.program[self.program_counter];
            self.program_counter += 1;

            self.do_token(token);
        }
    }

    fn tokenise(c: char) -> Option<BrainfuckToken> {
        if c == '<' {
            Some(BrainfuckToken::DecMemoryPointer)
        } else if c == '>' {
            Some(BrainfuckToken::IncMemoryPointer)
        } else if c == '+' {
            Some(BrainfuckToken::IncMemory)
        } else if c == '-' {
            Some(BrainfuckToken::DecMemory)
        } else if c == '.' {
            Some(BrainfuckToken::Output)
        } else if c == ',' {
            Some(BrainfuckToken::Input)
        } else if c == '[' {
            Some(BrainfuckToken::LoopStart)
        } else if c == ']' {
            Some(BrainfuckToken::LoopEnd)
        } else {
            None
        }
    }

    fn do_token(&mut self, token: BrainfuckToken) {
        match token {
            BrainfuckToken::IncMemoryPointer => self.memory_pointer += 1,
            BrainfuckToken::DecMemoryPointer => self.memory_pointer -= 1,
            BrainfuckToken::IncMemory => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(1);
            }
            BrainfuckToken::DecMemory => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(1);
            }
            BrainfuckToken::Output => print!("{}", self.memory[self.memory_pointer] as char),
            BrainfuckToken::Input => {
                // Read next available character character
                let c = std::io::stdin().lock().bytes().next().unwrap().unwrap();
                self.memory[self.memory_pointer] = c;
            }
            BrainfuckToken::LoopStart => self.callstack.push(self.program_counter),
            BrainfuckToken::LoopEnd => {
                if self.memory[self.memory_pointer] > 0 {
                    self.program_counter = self.callstack.top().expect("No matching '[' for ']'.");
                } else {
                    // Do nothing; go to next token
                    self.callstack.pop();
                }
            }
        }
    }
}
