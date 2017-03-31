mod overflow_behavior;
mod u8_editor;

pub use self::overflow_behavior::OverflowBehavior;
pub use self::u8_editor::U8Editor;

use std::io::Write;

pub trait MemoryCellEditor<T> {
    fn empty(&self) -> T;
    fn increment(&self, cell: &mut T);
    fn decrement(&self, cell: &mut T);
    fn read(&self, cell: &mut T, input: &mut Iterator<Item=u8>) -> Result<(), ()>;
    fn write(&self, cell: &T, output: &mut Write) -> Result<(), ()>;
    fn should_jump(&self, cell: &T) -> bool;
}
