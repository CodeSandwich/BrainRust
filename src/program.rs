use MemoryCellEditor;
use command::Command;
use parser;
use parser::ParseError;
use execution;
use execution::ExecutionError;
use std::io::Write;

pub struct Program {
    commands: Vec<Command>,
}

impl Program {
    pub fn new(source: &mut Iterator<Item=char>) -> Result<Self, ParseError> {
        Ok(Program {
            commands: parser::parse(source)?,
        })
    }

    pub fn execute<T, U: MemoryCellEditor<T>>(&self, cell_editor: &U, input: &mut Iterator<Item=u8>,
                output: &mut Write) -> Result<(), ExecutionError> {
        execution::execute(&self.commands, cell_editor, input, output)
    }
}
