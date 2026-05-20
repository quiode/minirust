//@ compile-flags: --minirust-tree-borrows --minirust-implicit-writes

// Copied from the Miri test suite and modified for MiniRust.
// This test shows that when implicit writes are enabled, Tree Borrows behaves more like Stacked
// Borrows, and no additional explicit write is needed to detect UB.

fn main() {
    let mut x = 0i32;
    let z = &mut x as *mut i32;
    x.do_bad();
    unsafe {
        let _oof = *z;
    }
}

trait Bad {
    fn do_bad(&mut self) {
        // who knows
    }
}

impl Bad for i32 {}
