use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use crate::{SafeUninitWrap, SafeUninit, ResizeUninit};

unsafe impl<T> SafeUninitWrap for Option<T> where T: SafeUninit {

    /// Create safe `Some` value which contains uninitialized value.
    fn safe_uninit() -> Self {
        Some(T::safe_uninit())
    }
}

impl<T> ResizeUninit for Vec<T> where T: SafeUninit {

    /// Resize the `Vec` as normal `resize_default` function does but instead of
    /// default values use uninitialized ones.
    fn resize_uninit(&mut self, new_len: usize) {
        self.resize_with(new_len, || T::safe_uninit())
    }
}

impl<T> ResizeUninit for VecDeque<T> where T: SafeUninit {

    /// Resize the `VecDeque` as normal `resize_default` function does but instead of
    /// default values use uninitialized ones.
    fn resize_uninit(&mut self, new_len: usize) {
        self.resize_with(new_len, || T::safe_uninit())
    }
}

unsafe impl<T> SafeUninitWrap for Box<T> where T: SafeUninit {

    /// `Box::new()` with safe uninitialized value.
    fn safe_uninit() -> Self {
        Box::new(T::safe_uninit())
    }
}

unsafe impl<T> SafeUninitWrap for Rc<T> where T: SafeUninit {

    /// `Rc::new()` with safe uninitialized value.
    fn safe_uninit() -> Self {
        Rc::new(T::safe_uninit())
    }
}
