// get_sign.rs
// Showcase how we manually can interface Rust to KLEE
//

#![no_std]
#![no_main]

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

// KLEE bindings

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // abort symbol caught by LLVM-KLEE
    unsafe { ll::abort() }
}

#[inline(always)]
pub fn klee_make_symbolic<T>(t: &mut T, name: &'static cstr_core::CStr) {
    unsafe {
        crate::ll::klee_make_symbolic(
            t as *mut T as *mut core::ffi::c_void,
            core::mem::size_of::<T>(),
            name.as_ptr() as *const cstr_core::c_char,
        )
    }
}

#[macro_export]
macro_rules! klee_make_symbolic {
    (&mut $id:expr, $name:expr) => {
        klee_make_symbolic(&mut $id, unsafe {
            cstr_core::CStr::from_bytes_with_nul_unchecked(concat!($name, "\0").as_bytes())
        })
    };
}

mod ll {
    //! Low level bindings
    extern "C" {
        pub fn abort() -> !;
        pub fn klee_make_symbolic(
            ptr: *mut core::ffi::c_void,    // pointer to the data
            size: usize,                    // size (in bytes)
            name: *const cstr_core::c_char, // pointer to zero-terminated C string
        );
    }
}

// A) Compile a Rust program for KLEE analysis.
//
// In order for KLEE analysis we need to generate LLVM-IR.
//
// > cargo rustc -v --example get_sign -- -C linker=true -C lto --emit=llvm-ir
//
// Here `rustc` tells cargo that we will call the compiler with arguments.
//
// `-v` that we want verbose output.
//
// Options after `--` is passed directly to to the compiler (`rustc`).
//
// `-C linker=true`, binds the linker to the `true` binary (a program that just returns).
// This way we can prevent the compiler to link to a binary (ugly right!).
//
// `-C --emit=llvm-ir`, to emit LLVM-IR in `.ll` form.
//
// `cargo` places the generated artifacts under the `target` directory.
//
// > ls target/debug/examples/
// get_sign-85c57be6132dac1d.d  get_sign-85c57be6132dac1d.ll  get_sign.d
//
// If you look at the compilation, above you find that `cargo` adds
// `extra-filename=-85c57be6132dac1d` (the hash might be different but)
// matches the hash for the `.ll` file.
//
// The hash is based on the setting for this particular build.
// This way, `cargo` keeps track of the builds, so you can several builds
// of the same files with different settings without name collisions.
//
// Now we can run KLEE on the generate LLVM-IR.
// (KLEE can read both `.bc` and `.ll` files, `.ll` files are human readable.)
//
// > klee target/debug/examples/get_sign-85c57be6132dac1d.ll
// KLEE: output directory is "/home/pln/courses/d7020e/klee_tutorial/target/debug/examples/klee-out-0"
// KLEE: Using Z3 solver backend
//
// KLEE: done: total instructions = 92
// KLEE: done: completed paths = 3
// KLEE: done: generated tests = 3
//
// (You need to give the right hash for the `.ll` file.)
//
// What was the generated hash.
//
// [your answer here]
//
// B) Inspecting the test cases.
//
// Figure out to run `ktest-tool` on the generated test cases.
// (Hint, it is just a matter of paths.)
//
// [your answer here]
//
// C) Replaying your test cases.
//
// Now we need to re-compile the `get_sign.rs` with bindings to
// the `libkleeRuntest`.
//
// The first thing we need to do is to generate an object file `.o`.
//
// > cd target/debug/examples
// > llc -filetype=obj -relocation-model=pic get_sign-85c57be6132dac1d.ll
//
// `llc` is the LLVM compiler, `-filetype=obj` to generate an object file and
// `-relocation-model=pic` is to obtain Position Independent Code (https://en.wikipedia.org/wiki/Position-independent_code)
// (This is needed since we don't know where it is going to be placed/linked later.)
//
// Verify that you now have an object file:
//
// > ls *.o
//
// [your answer here]
//
// Now we need to link it with the `libkleeRuntest`.
//
// > clang get_sign-85c57be6132dac1d.o -l kleeRuntest -o a.out
//
// (See `get_sign.c` for linking options if you don't have KLEE libs in default location.)
//
// Now we can replay the test case. However, we have compiled the example for `no_std`
// (as we target embedded applications).
//
// > KTEST_FILE=klee-last/test000001.ktest gdb a.out
//
// (gdb) get_sign
// Breakpoint 1 at 0x1165: file examples/get_sign.rs, line 9.
//
// Now run the code in the debugger. What path was triggered.
//
// [your answer here]
//
// Change to test000002, what path does it trigger.
//
// [your answer here]
//
// And finally change to test000003, what path was triggered.
//
// [your answer here]
//
// D) Remarks and conclusions.
//
// You have now successfully:
//
// - Generated LLVM-IR from Rust code
// - Run KLEE on Rust code to generate tests.
// - Linked with the 'kleeRuntest`.
// - Replayed code in `gdb'.
//
// It was a bit of a hassle right!
//
// In the next part of the lab we will:
//
// Introduce a Rust library for the LLVM bindings.
// Introduce a cargo sub-command for:
// - building and running KLEE
// - replaying test cases
//
// With this at hand, ergonomics using KLEE is improved.
// (Arguably better than the C/C++ integration.)
//
// So why did we not start directly with the `cargo klee`.
//
// Well this is course at advanced level, where you
// learn not only to use tools, but how the work.
//
// The library `klee-sys` basically extends on the
// primitive KLEE bindings in this file.
//
// The `cargo klee` command basically extends on the
// steps you have taken to compile and run the tools
// in this lab.
//
// If you grasp this lab, the rest is just coding.
// You may find and fix bugs in the `klee-sys`,
// improve and `cargo klee` etc.
//
// At that point you are `computer science master`!
