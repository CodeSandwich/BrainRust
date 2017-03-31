pub enum Command {
    MemFwd,
    MemBack,
    Inc,
    Dec,
    Write,
    Read,
    GoToIf(usize),
    GoTo(usize),
    End,
}
