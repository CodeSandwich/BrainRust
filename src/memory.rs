use MemoryCellEditor;

pub struct Memory<T> {
    positive_range: Vec<T>,
    negative_range: Vec<T>,
}

impl<T> Memory<T> {
    pub fn new() -> Memory<T> {
        Memory {
            positive_range: vec![],
            negative_range: vec![],
        }
    }

    pub fn get<U: MemoryCellEditor<T>>(&mut self, index: isize, cell_editor: &U) -> &mut T {
        let (range, actual_index) =
            if index >= 0 {
                (&mut self.positive_range, index as usize)
            } else {
                (&mut self.negative_range, (-index - 1) as usize)
            };
        for _ in range.len()..actual_index + 1 {
            range.push(cell_editor.empty())
        }
        &mut range[actual_index]
    }
}
