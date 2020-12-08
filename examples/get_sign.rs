// example that shows different types of path termination
// and their effect to KLEE test case generation
#![no_std]
#![no_main]

use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;

fn get_sign(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    if x < 0 {
        return -1;
    } else {
        return 1;
    }
}

#[no_mangle]
fn main() {
    let mut a: i32 = 0;
    klee_make_symbolic!(&mut a, "a");
    // std::process::exit(get_sign(a));
}

// > cargo rustc --example get_sign -- --emit=llvm-ir
