use memory_cell_editor::{MemoryCellEditor, OverflowBehavior};
use std::io::Write;
use std::u8::{MAX, MIN};

pub struct U8Editor {
    overflow_behavior: OverflowBehavior,
}

impl U8Editor {
    pub fn new(overflow_behavior: OverflowBehavior,) -> Self {
        U8Editor{
            overflow_behavior: overflow_behavior,
        }
    }
}

impl Default for U8Editor {
    fn default() -> Self {
        U8Editor::new(OverflowBehavior::Wrap)
    }
}

impl MemoryCellEditor<u8> for U8Editor {
    fn empty(&self) -> u8 {
        0
    }

    fn increment(&self, cell: &mut u8) {
        if *cell != MAX {
            *cell += 1;
        }
        else {
            match self.overflow_behavior {
                OverflowBehavior::Wrap => *cell = MIN,
                OverflowBehavior::Saturate => (),
            }
        }
    }

    fn decrement(&self, cell: &mut u8) {
        if *cell != MIN {
            *cell -= 1;
        }
        else {
            match self.overflow_behavior {
                OverflowBehavior::Wrap => *cell = MAX,
                OverflowBehavior::Saturate => (),
            }
        }
    }

    fn read(&self, cell: &mut u8, input: &mut Iterator<Item=u8>) -> Result<(), ()> {
        Ok(*cell = input.next().ok_or(())?)
    }

    fn write(&self, cell: &u8, output: &mut Write) -> Result<(), ()> {
        match output.write(&[*cell]) {
            Ok(1) => Ok(()),
            _ => Err(())
        }
    }

    fn should_jump(&self, cell: &u8) -> bool {
        *cell == 0
    }

}
