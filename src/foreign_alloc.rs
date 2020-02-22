use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use crate::{SafeUninitContent, SafeUninit, ResizeUninit};

unsafe impl<T> ResizeUninit for Vec<T> where T: SafeUninit {

    fn with_uninit(len: usize) -> Self {
        let mut v = Vec::with_capacity(len);
        v.resize_uninit(len);
        v
    }

    /// Resize the `Vec` as normal `resize_default` function does but instead of
    /// default values use uninitialized ones.
    fn resize_uninit(&mut self, new_len: usize) {
        self.resize_with(new_len, || T::safe_uninit())
    }
}

unsafe impl<T> ResizeUninit for VecDeque<T> where T: SafeUninit {

    fn with_uninit(len: usize) -> Self {
        let mut v = VecDeque::with_capacity(len);
        v.resize_uninit(len);
        v
    }

    /// Resize the `VecDeque` as normal `resize_default` function does but instead of
    /// default values use uninitialized ones.
    fn resize_uninit(&mut self, new_len: usize) {
        self.resize_with(new_len, || T::safe_uninit())
    }
}

unsafe impl<T> SafeUninitContent for Box<T> where T: SafeUninit {

    /// `Box::new()` with safe uninitialized value.
    fn uninit_content() -> Self {
        Box::new(T::safe_uninit())
    }
}

unsafe impl<T> SafeUninitContent for Rc<T> where T: SafeUninit {

    /// `Rc::new()` with safe uninitialized value.
    fn uninit_content() -> Self {
        Rc::new(T::safe_uninit())
    }
}
