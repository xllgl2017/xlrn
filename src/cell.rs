use std::ffi::{CStr, CString};
use crate::ffi;
use crate::ffi::WORK_SHEET;
use crate::pointer::CPointer;

pub struct SheetCell<'a> {
    sheet: &'a CPointer<WORK_SHEET>,
    row: u32,
    col: u32,
}

impl<'a> SheetCell<'a> {
    pub(crate) fn new(sheet: &'a CPointer<WORK_SHEET>, row: usize, col: usize) -> SheetCell<'a> {
        SheetCell {
            sheet,
            row: row as u32 + 1,
            col: col as u32 + 1,
        }
    }
    pub fn to_string(&self) -> String {
        let value = unsafe { ffi::WorkSheet_cell_string(self.sheet.as_ptr(), self.row, self.col) };
        let cell_value = unsafe { CStr::from_ptr(value) }.to_str().unwrap().to_string();
        unsafe { ffi::Excel_char_free(value) };
        cell_value
    }

    pub fn set_string(&self, value: impl ToString) {
        let ptr = CString::new(value.to_string()).unwrap();
        unsafe { ffi::WorkSheet_cell_set_string(self.sheet.as_mut_ptr(), self.row, self.col, ptr.as_ptr()) }
    }
}