pub struct Memory {
    positive_range: Vec<u8>,
    negative_range: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            positive_range: vec![],
            negative_range: vec![],
        }
    }

    pub fn get(&self, index: isize) -> u8 {
        let (range, actual_index) =
            if index >= 0 {
                (&self.positive_range, index)
            } else {
                (&self.negative_range, -index - 1)
            };
        range.get(actual_index as usize).cloned().unwrap_or(0)
    }

    pub fn set(&mut self, index: isize, value: u8) {
        let (range, actual_index) =
            if index >= 0 {
                (&mut self.positive_range, index as usize)
            } else {
                (&mut self.negative_range, (-index - 1) as usize)
            };
        for _ in range.len()..actual_index + 1 {
            range.push(0)
        }
        range[actual_index] = value;
    }
}
