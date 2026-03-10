use std::ptr::null_mut;
use crate::error::{XlrnError, XlrnResult};

pub struct CPointer<T: CFree<T>> {
    ptr: *mut T,
    auto_free: bool,
}
unsafe impl<T: CFree<T>> Send for CPointer<T> {}

unsafe impl<T: CFree<T>> Sync for CPointer<T> {}

impl<T: CFree<T>> CPointer<T> {
    // pub fn nullptr() -> Self {
    //     CPointer {
    //         ptr: null_mut(),
    //         auto_free: true,
    //     }
    // }

    pub fn new(ptr: *mut T) -> Self {
        CPointer {
            ptr,
            auto_free: true,
        }
    }

    pub fn new_checked(ptr: *mut T, e: XlrnError) -> XlrnResult<CPointer<T>> {
        if ptr.is_null() { return Err(e); };
        Ok(CPointer { ptr, auto_free: true })
    }
    // pub fn as_mut(&mut self) -> &mut *mut T { &mut self.ptr }
    pub fn as_mut_ptr(&self) -> *mut T { self.ptr }
    pub fn as_ptr(&self) -> *const T { self.ptr }
    // pub fn is_null(&self) -> bool { self.ptr.is_null() }

    // pub fn disable_auto_free(&mut self) { self.auto_free = false }
}

impl<T: CFree<T>> Drop for CPointer<T> {
    fn drop(&mut self) {
        T::free_ptr(self.ptr, self.auto_free);
        self.ptr = null_mut();
    }
}

pub trait CFree<T> {
    fn free_ptr(ptr: *mut T, free: bool);
}

