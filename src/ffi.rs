use std::os::raw::c_char;


#[repr(C)]
#[allow(non_camel_case_types)]
pub struct WORK_SHEET {
    _unused: [u8; 0],
}


#[repr(C)]
#[allow(non_camel_case_types)]
pub struct WORK_BOOK {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn Excel_open(path: *const c_char) -> *mut WORK_BOOK;
    pub fn WorkBook_save(book: *const WORK_BOOK, path: *const c_char);
    pub fn WorkBook_free(book: *mut WORK_BOOK);
    pub fn WorkBook_create() -> *mut WORK_BOOK;
    pub fn WorkBook_add_sheet(book: *mut WORK_BOOK, name: *const c_char) -> *mut WORK_SHEET;
    pub fn WorkBook_sheet(book: *const WORK_BOOK, index: usize) -> *mut WORK_SHEET;
    pub fn WorkSheet_free(sheet: *mut WORK_SHEET);
    pub fn WorkSheet_cell_string(sheet: *const WORK_SHEET, row: u32, col: u32) -> *mut c_char;
    pub fn WorkSheet_cell_set_string(sheet: *mut WORK_SHEET, row: u32, col: u32, value: *const c_char);
    pub fn Excel_char_free(s: *const c_char);
}

macro_rules! c_pointer_free {
    ($ty:ty, $free_fn:path) => {
        impl crate::pointer::CFree<$ty> for $ty {
            fn free_ptr(ptr: *mut $ty,free: bool){
                if !free { return; }
                if !ptr.is_null() { unsafe { $free_fn(ptr as _); } };
            }
        }
    };
}

c_pointer_free!(WORK_BOOK,WorkBook_free);
c_pointer_free!(WORK_SHEET,WorkSheet_free);