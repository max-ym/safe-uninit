`no_std` compatible.

Goal of the crate is to allow using safe uninitialized values. Many similar features
are already in the Rust `core` library as `MaybeUninit` and functions in standard types.
For example, currently (stable Rust 1.41) as nightly feature of a compiler one can see
`Box::new_uninit()` which allows to allocate memory with `MaybeUninit` value. Basically,
this new
crate allows creating not 'Maybe' but surely uninitialized values that are safe to use
despite they are uninitialized. Because of this they are directly presented as a value without
any wrappers like `MaybeUninit` and no requirement for unsafe block.

Main trait is `SafeUninit` which indicates the type which can be safely used without
initialization and without further wrappers. It is implemented for all primitive integer
types and their atomic variants, for fixed-size arrays of `SafeUninit` of up to 32 values
(but there is a way of creating bigger arrays),
for tuples of `SafeUninit` objects of up to 12 elements and for unit type `()`.

This crate is `no-std` but also implements traits for `alloc` types where appropriate.

Code samples:
```rust
let mut big_arr = [usize::safe_uninit(); 256];
for i in big_arr.iter() {
    println!("{}", i);
}
let mut small_arr: [u32; 32] = safe_uninit(); 


let rc: Rc<usize> = Rc::uninit_content();
let b: Box<i32> = Box::uninit_content();

let mut vec: Vec<usize> = Vec::with_capacity(100);
vec.resize_uninit(100); // Now Vec is filled with uninitialized content.

// Shorter variant:
let mut vec: Vec<usize> = Vec::with_uninit_len(100);
```
