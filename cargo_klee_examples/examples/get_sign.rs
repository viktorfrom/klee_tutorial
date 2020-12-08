// get_sign.rs
// Showcase how we automatically can interface Rust to KLEE
//

#![no_std]
#![no_main]

use klee_sys::klee_make_symbolic;
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
    get_sign(a);
}

// A) Compile a Rust program for KLEE analysis.
//
// The bindings to klee are abstracted into `klee-sys`.
// The panic handler into `panic_klee` (essentially KLEE `abort`).
// The tedious build process into `cargo klee`.
//
// > cargo klee -help
//
// The default is to generate LLVM-IR run KLEE on the generated LLVM-IR.
// This enables the `klee-analysis` feature of `klee-sys`.
//
// Using the `-v` option we see verbose output.
//
// > cargo klee -v --example get_sign
// "cargo" "rustc" "-v" "--features" "klee-analysis" "--example" "get_sign" "--color=always" "--" "-C" "linker=true" "-C" "lto" "--emit=llvm-ir"
// ...
//
// KLEE: Using Z3 solver backend
//
// KLEE: done: total instructions = 92
// KLEE: done: completed paths = 3
// KLEE: done: generated tests = 3
// This should be familiar to the build you did in the previous exercise.
//
// Now locate the generated tests. Give the relative path to `klee-last`.
//
// [your answer here]
//
// B) Replay test cases
//
// By giving the `-r` (or `--replay`), you can easily replay the test cases:
//
// > cargo klee -v -r -g --example get_sign
//
// (gdb) set environment KTEST_FILE=klee-last/test000001.ktest
// (gdb) break get_sign
// (gdb) run
//
// What path of the program does this path trigger?
//
// [your answer here]
//
// Just out of curiosity, you may test the other test cases as well...
//
// B) UX/IX
//
// Compare to the "stock KLEE for C" experience.
//
// How does `klee-sys` and `cargo-klee` score on a 0..5 scale?
//
// [your answer here]
//
// If below 5, what could be done to improve the UX/IX?
//
// [your answer here]
//
// C) Inner workings.
//
// Have a look on the source code of `klee-sys`.
// You will find that it is very similar to the code you saw in the
// previous exercise.
//
// Try to follow the `klee-analysis` feature.
// What modules in `klee-sys` does this feature enable?
//
// [your answer here]
//
// Have a look at the source code of `cargo klee`.
// (The actual sub-command is in the folder `cargo-klee`.)
//
// Try to figure out how the correct `.ll` file is determined.
// Which one will it pick, and why?
//
// [your answer here]
//
// Actually this is one of the "bad seeds" in the internal design.
// Seems there is no "stable" way of controlling/retrieving the "metadata"
// (i.e., the hashing). Ideas welcome!
