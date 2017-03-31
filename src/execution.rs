use command::Command;
use memory::Memory;
use memory_cell_editor::MemoryCellEditor;
use std::io::Write;

pub fn execute<T, U: MemoryCellEditor<T>>(commands: &Vec<Command>, cell_editor: &U, input: &mut Iterator<Item=u8>,
                  output: &mut Write) -> Result<(), ExecutionError> {
    let mut cmd_idx = 0;
    let mut mem_idx = 0;
    let mut mem = Memory::<T>::new();
    loop {
        match commands[cmd_idx] {
            Command::MemFwd => {
                mem_idx += 1
            },
            Command::MemBack => {
                mem_idx -= 1
            },
            Command::Inc => {
                cell_editor.increment(mem.get(mem_idx, cell_editor))
            },
            Command::Dec => {
                cell_editor.decrement(mem.get(mem_idx, cell_editor))
            },
            Command::Write => {
                cell_editor.write(mem.get(mem_idx, cell_editor), output).or(Err(ExecutionError::CannotWriteToOutput))?
            },
            Command::Read => {
                cell_editor.read(mem.get(mem_idx, cell_editor), input).or(Err(ExecutionError::CannotReadFromInput))?
            },
            Command::GoToIf(go_to_idx) => {
                if cell_editor.should_jump(mem.get(mem_idx, cell_editor)) {
                    cmd_idx = go_to_idx;
                    continue;
                }
            },
            Command::GoTo(go_to_idx) => {
                cmd_idx = go_to_idx;
                continue;
            },
            Command::End => {
                return Ok(())
            },
        }
        cmd_idx = cmd_idx + 1;
    }
}

pub enum ExecutionError {
    CannotWriteToOutput,
    CannotReadFromInput,
}
