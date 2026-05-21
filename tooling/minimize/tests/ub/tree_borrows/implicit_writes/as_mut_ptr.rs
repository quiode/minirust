//@ compile-flags: --minirust-tree-borrows --minirust-tree-borrows-implicit-writes

// Copied from the Miri test suite and modified for MiniRust.
// This code no longer works using implicit writes in tree borrows. This code tests that.

fn main() {
    let mut x: [u8; 3] = [1, 2, 3];

    let ptr = std::ptr::from_mut(&mut x);
    let a = unsafe { &mut *ptr };
    let b = unsafe { &mut *ptr };

    let _c = as_mut_ptr(a);
    let _v = *b;
}

pub fn as_mut_ptr(x: &mut [u8; 3]) -> *mut u8 {
    x as *mut [u8] as *mut u8
}
