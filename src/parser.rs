use command::Command;

pub fn parse(code: &mut Iterator<Item=char>) -> Result<Vec<Command>, ParseError> {
    let mut commands = vec![];
    let mut open_loops = vec![];
    for (index, symbol) in code.enumerate() {
        match symbol {
            '>' => commands.push(Command::MemFwd),
            '<' => commands.push(Command::MemBack),
            '+' => commands.push(Command::Inc),
            '-' => commands.push(Command::Dec),
            '.' => commands.push(Command::Write),
            ',' => commands.push(Command::Read),
            '[' => {
                open_loops.push(commands.len());

                commands.push(Command::End);
            },
            ']' => {
                let last_loop = open_loops.pop().ok_or(ParseError::NonexistentLoopClosed(index))?;
                commands.push(Command::GoTo(last_loop));
                commands[last_loop] = Command::GoToIf(commands.len());
            },
            _ => (),
        }
    }
    if !open_loops.is_empty() {
        return Err(ParseError::LoopNeverClosed(open_loops[0]));
    }
    commands.push(Command::End);
    Ok(commands)
}

pub enum ParseError {
    NonexistentLoopClosed(usize),
    LoopNeverClosed(usize),
}
