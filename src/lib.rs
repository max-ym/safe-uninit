//! Goal of the crate is to allow using safe uninitialized values. Many similar features
//! are already in the Rust `core` library as `MaybeUninit` and functions in standard types.
//! For example, currently (stable Rust 1.41) as nightly feature of a compiler one can see
//! `Box::new_uninit()` which allows to allocate memory with `MaybeUninit` value. Basically,
//! this new
//! crate allows creating not 'Maybe' but surely uninitialized values that are safe to use
//! despite they are uninitialized. Because of this they are directly presented as a value without
//! any wrappers like `MaybeUninit` and no requirement for unsafe block.
//!
//! Main trait is `SafeUninit` which indicates the type which can be safely used without
//! initialization and without further wrappers. It is implemented for all primitive integer
//! types and their atomic variants, for fixed-size arrays of `SafeUninit` of up to 32 values
//! (but there is a way of creating bigger arrays),
//! for tuples of `SafeUninit` objects of up to 12 elements and for unit type `()`.
//!
//! This crate is `no-std` but also implements traits for `alloc` types where appropriate.
//!
//! # Pointers
//! Pointers are safe to be uninitialized. Even if the values they are pointing to are not
//! `SafeUninit`.
//! Firstly, pointers are internally a plain number of type usize which is safe.
//! Secondly, dereferencing pointers is an unsafe operation anyway and even if pointer
//! with uninitialized address gets dereferenced this will be done under unsafe block
//! and programmer will be fully responsible for any consequences of using it.
//!
//! # Common Types That are Unsafe
//! These types are not safe to use uninitialized and one should use `MaybeUninit` instead.
//! ## bool
//! Boolean valid values are `true` and `false`. If boolean is internally
//! (as an example) stored as a byte which holds values different from 0 or 1 then this will
//! lead to unexpected behaviour and thus this type is not safe to use uninitialized. One
//! should use `MaybeUninit` for `bool`.
//! ## NonZero
//! Such types as `NonZeroI32` are unsafe to leave uninitialized. These types are assumed to
//! never be zero. Uninitialized value though can occur zero and this will cause undefined
//! behaviour.
//!
//! # Might be Unsafe
//! Here are listed types that can be unsafe and should be further investigated:
//!
//! * `char`
//! * `f32`
//! * `f64`

#![no_std]

extern crate alloc;
extern crate safe_uninit_derive;
pub use safe_uninit_derive::SafeUninit;

use core::mem::MaybeUninit;

/// Marks the type that is safe to use uninitialized. For example, if you create uninitialized `u32`
/// you still can use it and it would not cause any damage.

/// # Safety
/// This trait is unsafe because it is on the programmers side to identify the type as one that
/// is safe to use uninitialized. Failing to do this correctly can cause undefined
/// behaviour. On the other hand, this trait for the appropriate types allows faster initialization
/// in some cases when hard optimization is in concern.
///
/// # Fixed-Size Arrays
/// Current implementation allows uninitialized fixed-size arrays of `SafeUninit` of
/// up to 32 elements.
/// This is due `stable` compiler lacking `const generics` and will be unlimited when feature
/// gets stabilized. Still, one can initialize big arrays with easy-enough syntax.
/// ```
/// # use safe_uninit::SafeUninit;
/// // It is possible to omit the type if compiler is able to infer it.
/// // In this case it cannot so we just tell the type.
/// let mut small_arr: [usize; 32] = SafeUninit::safe_uninit();
/// for i in &small_arr {
///     println!("{}", i);
/// }
///
/// // The first example will not work for this array as it's len > 32.
/// // Still, it can be (un)initialized.
/// // This variant is valid for small arrays too.
/// let mut big_arr = [usize::safe_uninit(); 256];
/// for i in big_arr.iter() {
///     println!("{}", i);
/// }
///
/// // Also, small arrays can be (un)initialized like this, but big cannot.
/// let mut small_arr = <[usize; 32]>::safe_uninit();
/// for i in &small_arr {
///     println!("{}", i);
/// }
/// ```
///
/// # Tuples
/// Similar to arrays, tuples can be instantiated with uninitialized values in cases where tuples
/// have no more than 12 values.
/// ```
/// # use safe_uninit::*;
/// let tuple = <(usize, *mut f32, ())>::safe_uninit();
/// let tuple: (usize, *mut f32, ()) = SafeUninit::safe_uninit();
/// ```
///
/// # Custom Types
/// One can implement `SafeUninit` for custom types and use them as any other type from these
/// examples.
///
/// # Shorthand
/// The trait also has shorthand call. Shorter form of `SafeUninit::safe_uninit()` is just
/// `safe_uninit()`. Though this function should be imported from this crate.
pub unsafe trait SafeUninit: Sized {

    fn safe_uninit() -> Self {
        unsafe {
            MaybeUninit::uninit().assume_init()
        }
    }
}

/// Similar to `SafeUninit`.
/// This trait intended to be implemented for types like `Rc` or `Box` and instead mean that
/// content that this object holds inside is uninitialized (and not `Rc` or `Box` itself).
/// ```
/// # use safe_uninit::*;
/// use std::rc::Rc;
///
/// let rc: Rc<usize> = Rc::uninit_content();
/// let b: Box<i32> = Box::uninit_content();
/// ```
/// # Option
/// `Option` can also be initialized to `Some` variant with uninitialized value inside.
/// ```
/// # use safe_uninit::*;
///
/// // Code does the same:
/// let option: Option<usize> = Option::Some(SafeUninit::safe_uninit());
/// let option: Option<usize> = Option::uninit_content();
/// ```
pub unsafe trait UninitContent: Sized {

    fn uninit_content() -> Self;
}

/// To be used with `Vec`-like types. Adds `Vec` a capability to resize it's content while leaving
/// new values uninitialized.
/// ```
/// # use safe_uninit::*;
/// use std::vec::Vec;
///
/// let mut vec: Vec<usize> = Vec::with_capacity(100);
/// vec.resize_uninit(100); // Now Vec is filled with uninitialized content.
///
/// // Shorter variant:
/// let mut vec: Vec<usize> = Vec::with_uninit_len(100);
/// ```
pub unsafe trait ResizeUninit {

    /// Allocate collection with given length containing only uninitialized values.
    fn with_uninit_len(len: usize) -> Self;

    /// Resize collection. If the new length is greater - leave all new elements uninitialized.
    fn resize_uninit(&mut self, new_len: usize);
}

/// Shorthand for types that are `SafeUninit`.
/// ```
/// # use safe_uninit::*;
/// // This code does the same:
/// let arr: [usize; 32] = safe_uninit();
/// let arr: [usize; 32] = SafeUninit::safe_uninit();
///
/// let arr: [usize; 256] = [SafeUninit::safe_uninit(); 256];
/// let arr: [usize; 256] = [safe_uninit(); 256];
/// ```
pub fn safe_uninit<T: SafeUninit>() -> T {
    T::safe_uninit()
}

/// Contains implementation for foreign types from `core`.
mod foreign_core;

/// Contains implementation for foreign types from `alloc`.
mod foreign_alloc;
