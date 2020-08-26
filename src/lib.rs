#![crate_name="linux_syscalls"]
#![crate_type="lib"]

#![feature(asm)]
#![deny(warnings)]
#![no_std]
#![allow(unused_mut)]

#[cfg(all(target_os="linux", target_arch="x86_64"))]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;


