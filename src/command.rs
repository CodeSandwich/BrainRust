pub enum Command {
    MemFwd,
    MemBack,
    Inc,
    Dec,
    Write,
    Read,
    GoToIfZero(usize),
    GoTo(usize),
    End,
}
