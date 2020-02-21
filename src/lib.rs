//! Object of the crate is to allow using safe uninitialized values while Rust Standard Library
//! gets stabilized features for uninitialized values.
//! For example, currently (Rust 1.41) on nightly version of a compiler one can see standard
//! Box::new_uninit() which allows to allocate memory for MaybeUninit values. Basically, this new
//! crate allows creating not 'Maybe' but surely uninitialized values that are safe to use
//! despite they are uninitialized.
//!
//! Main trait is SafeUninit that indicated the type which can be safely used without
//! initialization and without further wrappers. It is implemented for all primitive numerical
//! types and their atomic variants, for fixed-size arrays of SafeUninit of up to 32 values,
//! for tuples of SafeUninit objects of up to 12 elements and for unit type `()`.
//!
//! This crate also implements traits for `alloc` types where appropriate.
//!
//! # Pointers
//! Pointers are safe to have uninitialized. Even if the values they are pointing to are unsafe.
//! Firstly, pointers are internally a plain number of type usize which is safe.
//! Secondly, dereferencing pointers is an unsafe operation anyway and even if pointer
//! with uninitialized address gets dereferenced this will be done under unsafe block
//! and programmer will be fully responsible for any consequences of using it.
//!
//! # Common types that are unsafe
//! ## bool
//! Boolean valid values are `true` and `false`. If boolean is internally
//! (as an example) stored as a byte which holds values different from 0 or 1 then this will
//! lead to unexpected behaviour and thus this type is not safe to use uninitialized.
//! ## NonZero
//! Such types as NonZeroI32 are unsafe to leave uninitialized. These types are assumed to
//! never be zero. Uninitialized value though can occur zero and this will cause undefined
//! behaviour.
//!
//! # Types that might be unsafe
//! Here are listed types that can be unsafe and should be further investigated.
//!
//! * char

#![no_std]

extern crate alloc;

use core::mem::MaybeUninit;

/// Marks the type that is safe to use uninitialized. For example, if you create uninitialized u32
/// you still can use it and it would not cause any damage.
/// This trait is unsafe because it is on the programmers side to identify this type as one that
/// is safe to use uninitialized. Failing to do this correctly can actually cause undefined
/// behaviour. On the other hand, this trait for the appropriate types allows faster initialization
/// when hard optimization is in concern.
pub unsafe trait SafeUninit: Sized {

    fn safe_uninit() -> Self {
        unsafe {
            MaybeUninit::uninit().assume_init()
        }
    }
}

/// Similar to SafeUninit.
/// This trait can be implemented for types like `Rc` or `Box` and instead mean that
/// value that this object hold are uninitialized (and not `Rc` or `Box` itself).
pub unsafe trait SafeUninitWrap: Sized {

    fn safe_uninit() -> Self;
}

/// To be used with `Vec`-like types. Adds `Vec` a capability to resize it's content while leaving
/// new values uninitialized.
pub trait ResizeUninit {

    fn resize_uninit(&mut self, new_len: usize);
}

/// Contains implementation for foreign types from `core`.
mod foreign_core;

/// Contains implementation for foreign types from `alloc`.
mod foreign_alloc;
