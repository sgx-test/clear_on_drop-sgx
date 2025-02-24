#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "nightly", feature(llvm_asm, i128_type, specialization))]
#![deny(missing_docs)]

//! Helpers for clearing sensitive data on the stack and heap.
//!
//! Some kinds of data should not be kept in memory any longer than
//! they are needed. For instance, cryptographic keys and intermediate
//! values should be erased as soon as they are no longer needed.
//!
//! The Rust language helps prevent the accidental reading of leftover
//! values on the stack or the heap; however, means outside the program
//! (for instance a debugger, or even physical access to the hardware)
//! can still read the leftover values. For long-lived processes, key
//! material might be found in the memory long after it should have been
//! discarded.
//!
//! This crate provides two mechanisms to help minimize leftover data.
//!
//! The `ClearOnDrop` wrapper holds a mutable reference to sensitive
//! data (for instance, a cipher state), and clears the data when
//! dropped. While the mutable reference is held, the data cannot be
//! moved, so there won't be leftovers due to moves; the wrapper itself
//! can be freely moved. Alternatively, it can hold data on the heap
//! (using a `Box<T>`, or possibly a similar which allocates from a
//! `mlock`ed heap).
//!
//! The `clear_stack_on_return` function calls a closure, and after it
//! returns, overwrites several kilobytes of the stack. This can help
//! overwrite temporary variables used by cryptographic algorithms, and
//! is especially relevant when running on a short-lived thread, since
//! the memory used for the thread stack cannot be easily overwritten
//! after the thread terminates.
//!
//! # Preventing compiler optimizations
//!
//! If the compiler determines the data is not used after being cleared,
//! it could elide the clearing code. Aditionally, the compiler could
//! inline a called function and the stack clearing code, using separate
//! areas of the stack for each. This crate has three mechanisms which
//! prevent these unwanted optimizations, selected at compile time via
//! cargo features.
//!
//! The fastest mechanism uses inline assembly, which is only available
//! on nightly Rust. It is enabled through the `nightly` feature, and
//! does not need a working C compiler.
//!
//! The second mechanism, which is the default, uses a call to a dummy
//! C function. It works on stable Rust, but needs a working C compiler.
//!
//! The third mechanism is a fallback, which attempts to confuse the
//! optimizer through the use of atomic instructions. It should not be
//! used unless necessary, since it's less reliable. It is enabled by
//! the `no_cc` feature, works on stable Rust, and does not need a C
//! compiler.

#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(test)]
extern crate core;

pub mod clear;
mod clear_on_drop;
mod clear_stack_on_return;
mod fnoption;
mod hide;

pub use clear_on_drop::*;
pub use clear_stack_on_return::*;
