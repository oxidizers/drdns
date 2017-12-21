//! `lib.rs`: Placeholder to ensure code compiles

#![deny(warnings)]
#![allow(dead_code)]

extern crate errno;
extern crate libc;

/// The modules listed below have had a first pass to convert them into
/// proper Rust modules. Any modules not listed below are not presently
/// covered by CI to ensure the code compiles.
mod cdb;
mod alloc;
mod buffer;
mod byte;
mod stralloc;
mod strerr;
mod tai;
mod taia;
mod uint16;
mod uint32;
