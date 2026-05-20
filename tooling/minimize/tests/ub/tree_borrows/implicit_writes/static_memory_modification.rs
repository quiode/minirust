//@ compile-flags: --minirust-tree-borrows --minirust-implicit-writes

// Copied from the Miri test suite and modified for MiniRust.
// Tests that inserting an implicit write to a read-only allocation generates the correct error message.

static X: usize = 5;

#[allow(mutable_transmutes)]
fn main() {
    let x = unsafe { std::mem::transmute::<&usize, &mut usize>(&X) };
    foo(x);
}

fn foo(_x: &mut usize) {}
