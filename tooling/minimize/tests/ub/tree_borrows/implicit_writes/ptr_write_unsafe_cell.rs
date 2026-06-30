//@ compile-flags: --minirust-tree-borrows --minirust-tree-borrows-implicit-writes

// Copied from the Miri test suite and modified for MiniRust.
// Same test as `ptr_write.rs` but with `UnsafeCell`.

use std::cell::UnsafeCell;

fn main() {
    let mut x: UnsafeCell<u8> = 0u8.into();
    let ptr = x.get();
    let res = dereference(&mut x, ptr);
    let _ = *res.get_mut();
}

fn dereference<T>(x: T, y: *mut u8) -> T {
    let _ = unsafe { *y };
    x
}
