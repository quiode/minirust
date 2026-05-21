//@ compile-flags: --minirust-tree-borrows

// Copied from the Miri test suite and modified for MiniRust.
// Shows that without implicit writes, `ptr_write.rs`, `ptr_write_unsafe_cell.rs`,
// `ptr_write_box.rs`, `as_mut_ptr.rs` would pass.

use std::cell::UnsafeCell;

fn main() {
    normal();
    unsafe_cell();
    box_test();
    as_mut_ptr_test()
}

fn normal() {
    let mut x = 0u8;
    let ptr = &raw mut x;
    let res = dereference(&mut x, ptr);
    let _ = *res;
}

fn unsafe_cell() {
    let mut x: UnsafeCell<u8> = 0u8.into();
    let ptr = x.get();
    let res = dereference(&mut x, ptr);
    let _ = *res.get_mut();
}

fn box_test() {
    let mut x = 0u8;
    let ptr = &raw mut x;
    // Use transmute to create a Box without going through Box::new.
    let b = unsafe { std::mem::transmute::<*mut u8, Box<u8>>(ptr) };
    let res = dereference(b, ptr);
    std::mem::forget(res);
}

fn dereference<T>(x: T, y: *mut u8) -> T {
    let _ = unsafe { *y };
    x
}

fn as_mut_ptr_test() {
    let mut x: [u8; 3] = [1, 2, 3];

    let ptr = std::ptr::from_mut(&mut x);
    let a = unsafe { &mut *ptr };
    let b = unsafe { &mut *ptr };

    let _c = as_mut_ptr(a);
    let _v = *b;
}

// This should be the same as the implementation for slice, as this is known to cause errors
// with implicit writes and we want to test that behavior here.
pub fn as_mut_ptr(x: &mut [u8; 3]) -> *mut u8 {
    x as *mut [u8] as *mut u8
}
