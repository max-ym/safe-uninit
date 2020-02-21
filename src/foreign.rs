use crate::{SafeUninit};
use core::sync::atomic::*;

unsafe impl<T0, T1> SafeUninit for (T0, T1)
    where T0: SafeUninit, T1: SafeUninit {}

unsafe impl<T0, T1, T2> SafeUninit for (T0, T1, T2)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit {}

unsafe impl<T0, T1, T2, T3> SafeUninit for (T0, T1, T2, T3)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4> SafeUninit for (T0, T1, T2, T3, T4)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5> SafeUninit for (T0, T1, T2, T3, T4, T5)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6> SafeUninit for (T0, T1, T2, T3, T4, T5, T6)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7> SafeUninit for (T0, T1, T2, T3, T4, T5, T6, T7)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit, T7: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> SafeUninit for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit, T7: SafeUninit, T8: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> SafeUninit for
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit, T7: SafeUninit, T8: SafeUninit, T9: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SafeUninit for
(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit, T7: SafeUninit, T8: SafeUninit, T9: SafeUninit,
          T10: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> SafeUninit for
(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit, T7: SafeUninit, T8: SafeUninit, T9: SafeUninit,
          T10: SafeUninit, T11: SafeUninit {}

unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> SafeUninit for
(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
    where T0: SafeUninit, T1: SafeUninit, T2: SafeUninit, T3: SafeUninit, T4: SafeUninit,
          T5: SafeUninit, T6: SafeUninit, T7: SafeUninit, T8: SafeUninit, T9: SafeUninit,
          T10: SafeUninit, T11: SafeUninit, T12: SafeUninit {}

unsafe impl<T> SafeUninit for *const T {}

unsafe impl<T> SafeUninit for *mut T {}

unsafe impl<T> SafeUninit for AtomicPtr<T> {}

macro_rules! array_impls {
    ($($N: literal)+) => {
        $(
            unsafe impl<T: SafeUninit> SafeUninit for [T; $N] {}
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

macro_rules! impl_safe(
    ($ty: ty) => {
        unsafe impl SafeUninit for $ty {}
    }
);

impl_safe!(u8);
impl_safe!(u16);
impl_safe!(u32);
impl_safe!(u64);
impl_safe!(u128);
impl_safe!(i8);
impl_safe!(i16);
impl_safe!(i32);
impl_safe!(i64);
impl_safe!(i128);
impl_safe!(f32);
impl_safe!(f64);
impl_safe!(usize);
impl_safe!(isize);
impl_safe!(());

impl_safe!(AtomicU8);
impl_safe!(AtomicU16);
impl_safe!(AtomicU32);
impl_safe!(AtomicU64);
impl_safe!(AtomicI8);
impl_safe!(AtomicI16);
impl_safe!(AtomicI32);
impl_safe!(AtomicI64);
impl_safe!(AtomicUsize);
impl_safe!(AtomicIsize);

#[cfg(std)]
mod std {
    use std::vec::Vec;
    use std::collections::*;
    use std::boxed::Box;
    use crate::SafeUninit;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex, RwLock};

    unsafe impl<T> SafeUninit for Option<T> where T: SafeUninit {

        /// Create safe `Some` value which contains uninitialized value.
        fn safe_uninit() -> Self {
            Some(T::safe_uninit())
        }
    }

    impl<T> Vec<T> where T: SafeUninit {

        /// Resize the `Vec` as normal `resize_default` function does but instead of
        /// default values use uninitialized ones.
        pub fn resize_uninit(&mut self, new_len: usize) {
            self.resize(new_len, T::safe_uninit())
        }
    }

    impl<T> VecDeque<T> where T: SafeUninit {

        /// Resize the `VecDeque` as normal `resize_default` function does but instead of
        /// default values use uninitialized ones.
        pub fn resize_uninit(&mut self, new_len: usize) {
            self.resize(new_len, T::safe_uninit())
        }
    }

    impl<T> Box<T> where T: SafeUninit {

        /// `Box::new()` with safe uninitialized value.
        pub fn new_safe_uninit() -> Self {
            Box::new(T::safe_uninit())
        }
    }

    impl<T> Rc<T> where T: SafeUninit {

        /// `Rc::new()` with safe uninitialized value.
        pub fn new_safe_uninit() -> Self {
            Rc::new(T::safe_uninit())
        }
    }

    impl<T> Arc<T> where T: SafeUninit {

        /// `Arc::new()` with safe uninitialized value.
        pub fn new_safe_uninit() -> Self {
            Arc::new(T::safe_uninit())
        }
    }

    impl<T> Mutex<T> where T: SafeUninit {

        /// `Mutex::new()` with safe uninitialized value.
        pub fn new_safe_uninit() -> Self {
            Mutex::new(T::safe_uninit())
        }
    }

    impl<T> RwLock<T> where T: SafeUninit {

        /// `RwLock::new()` with safe uninitialized value.
        pub fn new_safe_uninit() -> Self {
            RwLock::new(T::safe_uninit())
        }
    }
}
