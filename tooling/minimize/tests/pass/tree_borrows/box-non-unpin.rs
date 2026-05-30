//@ compile-flags: --minirust-tree-borrows

// In `reborrow_settings`, if a Box<T> is !Unpin, previously we did NOT leave the function early and did NOT return None.
// This behavior differs to Miri, where we do return None.
// This tests failed using the old behavior, but passed in Miri.
// The behavior has been changed in https://github.com/minirust/minirust/issues/291 and now matches the one in Miri.

extern crate intrinsics;
use intrinsics::*;

use std::marker::PhantomPinned;

pub struct NotUnpin(i32, PhantomPinned);

fn f(_b: Box<NotUnpin>, xraw: *mut i32) {
    unsafe { *xraw = 42 };
    std::mem::forget(_b);
}

fn main() {
    // Box::new needs the global allocator which is unsupported; allocate manually.
    let ptr = unsafe { allocate(4, 4) } as *mut NotUnpin;
    unsafe { ptr.write(NotUnpin(0, PhantomPinned)) };

    let xraw: *mut i32 = unsafe { std::ptr::addr_of_mut!((*ptr).0) };
    let b: Box<NotUnpin> = unsafe { std::mem::transmute(ptr) };

    f(b, xraw);
    assert!(unsafe { *xraw } == 42);
    unsafe { deallocate(ptr as *mut u8, 4, 4) }; // Manually deallocate the memory we allocated for the Box.
}
