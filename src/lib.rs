#![crate_name = "linux_syscalls"]
#![crate_type = "lib"]
#![feature(asm)]
#![deny(warnings)]
#![no_std]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//Made the Unsafe code hidden from the user
#[macro_use]
pub(crate) mod macros;

pub mod types;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[path = "platform/linux-x86_64/mod.rs"]
pub mod platform;
