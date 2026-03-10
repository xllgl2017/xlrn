use crate::error::{XlrnError, XlrnResult};
use crate::ffi::WORK_BOOK;
use crate::pointer::CPointer;
use crate::ffi;
use std::ffi::CString;
use std::path::Path;
use crate::sheet::WorkSheet;

pub struct WorkBook {
    book: CPointer<WORK_BOOK>,
}

impl WorkBook {
    pub fn create_book() -> (WorkBook, WorkSheet) {
        let book = WorkBook { book: CPointer::new(unsafe { ffi::WorkBook_create() }) };
        let sheet = WorkSheet::new(CPointer::new(unsafe { ffi::WorkBook_sheet(book.book.as_ptr(), 0) }));
        (book, sheet)
    }

    pub fn open(path: impl AsRef<Path>) -> XlrnResult<WorkBook> {
        let path = path.as_ref();
        if !path.exists() { return Err(XlrnError::FileNotFound); }
        let path = CString::new(path.display().to_string())?;
        let book = WorkBook { book: CPointer::new(unsafe { ffi::Excel_open(path.as_ptr()) }) };
        Ok(book)
    }


    pub fn add_sheet(&self, name: &str) -> XlrnResult<WorkSheet> {
        let c_name = CString::new(name)?;
        let sheet = unsafe { ffi::WorkBook_add_sheet(self.book.as_mut_ptr(), c_name.as_ptr()) };
        Ok(WorkSheet::new(CPointer::new(sheet)))
    }

    pub fn get_sheet(&self, index: usize) -> XlrnResult<WorkSheet> {
        let sheet = unsafe { ffi::WorkBook_sheet(self.book.as_ptr(), index) };
        Ok(WorkSheet::new(CPointer::new_checked(sheet, XlrnError::WorkSheetNull)?))
    }

    pub fn save(&self, path: &str) -> XlrnResult<()> {
        let c_path = CString::new(path)?;
        unsafe {
            ffi::WorkBook_save(self.book.as_ptr(), c_path.as_ptr());
        }
        Ok(())
    }
}