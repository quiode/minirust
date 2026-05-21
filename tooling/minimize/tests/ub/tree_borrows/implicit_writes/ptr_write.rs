//@ compile-flags: --minirust-tree-borrows --minirust-tree-borrows-implicit-writes

// Copied from the Miri test suite and modified for MiniRust.
// Tests that UB is detected for reading from a mutable reference by another pointer
// (as there could be a write on that mutable reference now).

fn main() {
    let mut x = 0u8;
    let ptr = &raw mut x;
    let res = dereference(&mut x, ptr);
    let _ = *res;
}

fn dereference<T>(x: T, y: *mut u8) -> T {
    let _ = unsafe { *y };
    x
}
