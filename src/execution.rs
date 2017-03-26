use command::Command;
use memory::Memory;
use std::io::Write;
use std::u8::{MAX, MIN};

pub fn execute(commands: &Vec<Command>, input: &mut Iterator<Item=u8>, output: &mut Write)
               -> Result<(), ExecutionError> {
    let mut cmd_idx = 0;
    let mut mem_idx = 0;
    let mut mem = Memory::new();
    loop {
        match commands[cmd_idx] {
            Command::MemFwd => {
                mem_idx += 1;
            },
            Command::MemBack => {
                mem_idx -= 1;
            },
            Command::Inc => {
                let value = mem.get(mem_idx);
                let value = if value == MAX { MIN } else { value + 1 };
                mem.set(mem_idx, value);
            },
            Command::Dec => {
                let value = mem.get(mem_idx);
                let value = if value == MIN { MAX } else { value - 1 };
                mem.set(mem_idx, value);
            },
            Command::Write => {
                let value = [mem.get(mem_idx)];
                match output.write(&value) {
                    Ok(0) | Err(_) => return Err(ExecutionError::CannotWriteToOutput),
                    _ => ()
                }
            },
            Command::Read => {
                let value = input.next().ok_or(ExecutionError::CannotReadFromInput)?;
                mem.set(mem_idx, value);
            },
            Command::GoToIfZero(go_to_idx) => {
                if mem.get(mem_idx) == 0 {
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
