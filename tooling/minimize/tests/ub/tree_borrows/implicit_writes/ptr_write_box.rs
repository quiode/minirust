//@ compile-flags: --minirust-tree-borrows --minirust-implicit-writes

// Copied from the Miri test suite and modified for MiniRust.
// Same test as `ptr_write.rs` but with `Box`.

fn main() {
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
