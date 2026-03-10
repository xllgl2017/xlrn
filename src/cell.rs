use std::ffi::{CStr, CString};
use crate::error::XlrnResult;
use crate::ffi;
use crate::ffi::{SHEET_CELL, WORK_SHEET};
use crate::pointer::CPointer;

pub struct SheetCell {
    cell: SHEET_CELL,
}

impl SheetCell {
    pub(crate) fn new(sheet: &CPointer<WORK_SHEET>, row: usize, col: usize) -> SheetCell {
        SheetCell {
            cell: SHEET_CELL {
                sheet: sheet.as_mut_ptr(),
                row: row as u32 + 1,
                col: col as u32 + 1,
            }
        }
    }
    pub fn to_string(&self) -> XlrnResult<String> {
        let value = unsafe { ffi::Cell_string(&self.cell) };
        let cell_value = unsafe { CStr::from_ptr(value) }.to_str()?.to_string();
        unsafe { ffi::Excel_char_free(value) };
        Ok(cell_value)
    }

    pub fn set_string(&self, value: impl ToString) -> XlrnResult<()> {
        let ptr = CString::new(value.to_string())?;
        unsafe { ffi::Cell_set_string(&self.cell, ptr.as_ptr()) };
        Ok(())
    }

    pub fn set_font(&self, font_name: impl AsRef<str>, font_size: u32, bold: bool, italic: bool) -> XlrnResult<()> {
        let font_name = CString::new(font_name.as_ref())?;
        unsafe { ffi::Cell_set_font(&self.cell, font_name.as_ptr(), font_size, bold, italic) };
        Ok(())
    }

    pub fn set_hori_align(&self, align: CellAlign) {
        unsafe { ffi::Cell_set_align(&self.cell, align, CellAlign::Left) }
    }

    pub fn set_ver_align(&self, align: CellAlign) {
        unsafe { ffi::Cell_set_align(&self.cell, CellAlign::Top, align) };
    }

    pub fn set_align(&self, hori: CellAlign, veri: CellAlign) {
        unsafe { ffi::Cell_set_align(&self.cell, hori, veri) }
    }
}

#[repr(C)]
pub enum CellAlign {
    Left = 0,
    Center = 1,
    Right = 2,
    Top = 3,
    Bottom = 4,
}