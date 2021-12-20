#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;
pub mod api;
pub mod handlers;
pub mod kernel;
pub mod sys;
pub use alloc::*;
use linked_list_allocator::LockedHeap;
set_alloc_error_hook!();
#[global_allocator]
#[allow(non_upper_case_globals)]
pub static System: LockedHeap = LockedHeap::empty();
