use crate::cell::SheetCell;
use crate::ffi::WORK_SHEET;
use crate::pointer::CPointer;

pub struct WorkSheet {
    sheet: CPointer<WORK_SHEET>,
}

impl WorkSheet {
    pub(crate) fn new(sheet: CPointer<WORK_SHEET>) -> WorkSheet {
        WorkSheet {
            sheet
        }
    }
    pub fn cell(&self, row: usize, col: usize) -> SheetCell<'_> {
        SheetCell::new(&self.sheet, row, col)
    }
}
