pub struct LoxError {
    line: usize,
    message: String,
    position: usize,
}

pub enum RuntimeResult {
    Safe,
    LexicalError,
    ParserError,
    InterpreterError,
}

impl LoxError {
    pub fn error(line: usize, message: String, position: usize) -> LoxError {
        LoxError {
            line,
            message,
            position,
        }
    }

    pub fn report(&self) {
        println!(
            "[line {}, position {}] Error: {}",
            self.line, self.position, self.message
        );
    }
}
