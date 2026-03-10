use crate::cell::SheetCell;
use crate::ffi;
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
    pub fn cell(&self, row: usize, col: usize) -> SheetCell {
        SheetCell::new(&self.sheet, row, col)
    }

    pub fn set_column_width(&self, col: usize, width: u32) {
        unsafe { ffi::WorkSheet_set_column_width(self.sheet.as_mut_ptr(), col as u32 + 1, width) };
    }

    pub fn set_row_height(&self, row: usize, height: u32) {
        unsafe { ffi::WorkSheet_set_row_height(self.sheet.as_mut_ptr(), row as u32 + 1, height) };
    }
}
