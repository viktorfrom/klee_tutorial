// cyccnt.rs

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_klee;
use klee_sys::klee_make_symbolic;

#[no_mangle]
fn main() {
    let mut core = cortex_m::Peripherals::take().unwrap();
    klee_make_symbolic!(&mut core, "core"); 
    // core.DCB.enable_trace();
    // core.DWT.enable_cycle_counter();

    let start: u32 = core.DWT.cyccnt.read();

    // some code to measure
    // ...

    let end = core.DWT.cyccnt.read();

    let _time = end - start;
}

// A) Running KLEE on embedded code:
//
// Let's now revisit the time measure example from `rtic_f4xx_nucleo`.
// For simplicity we omit the `rtic` specifics.
//
// `cortex-m` is an API to the ARM Cortex core peripherals.
// (Under RTIC it was passed to the `init::Context` as `core`.)
//
// In this example we access it directly.
// Since Rust forbids mutable aliasing, we cannot `take` it twice,
// but more about that later...
//
// Let's focus on the access to the core peripherals, and
// the relation to the `klee-analysis` feature.
//
// In the Rust embedded ecosystem, all hardware registers
// are accessed through `vcell`, which ensure `volatile access`.
// (This means that reads and writes goes to memory.)
//
// We override `vcell` by a custom implementation:
//
// Looking at the `Cargo.toml` we find:
//
// [patch.crates-io]
// cortex-m = { git = "https://github.com/perlindgren/cortex-m.git", branch = "trustit" }
// vcell = { git = "https://github.com/perlindgren/vcell.git", branch = "trustit" }
//
// and
//
// [features]
// klee-analysis = [
//     "klee-sys/klee-analysis",
//     "cortex-m/klee-analysis",
//     "vcell/klee-analysis"
// ]
//
// The `klee-analysis` feature is propagated to `vcell`.
// When active:
// - writes are suppressed (they have no effect), and
// - reads generate a fresh (new) unique symbolic value.
//
// In the code above Line 15, we get one symbolic value for `DWT.cyccnt.read()`,
// and in Line 20, we get another unique symbolic value.
//
// Among other things `cortex-m` provides an API to read/write the `PRIMASK`,
// core register. (This controls the global interrupt enable/disable).
//
// We override the `cortex-m` crate, and propagate the `klee-analysis` feature.
// Similarly to `vcell`, when the `klee-analysis` feature is active:
// - writes to PRIMASK are suppressed, and
// - reads generate a fresh (new) unique value.
//
// In the code above Line 11, `cortex-m` executes the `take()` operation
// in a "critical section". In more detail:
// It first reads the `PRIMASK`, if false (we are not already in a critical section)
// it sets the `PRIMASK`.
//
// Now let's run the analysis:
//
// > cargo klee --example cyccnt --release
//
// KLEE: Using Z3 solver backend
// KLEE: WARNING: undefined reference to function: rust_eh_personality
//
// KLEE: done: total instructions = 38
// KLEE: done: completed paths = 1
// KLEE: done: generated tests = 1
//
// Only one path was found (no error occurred along the path).
//
// > ktest-tool target/release/examples/klee-last/test000001.ktest
//
// ktest file : 'target/release/examples/klee-last/test000001.ktest'
// args       : ['/home/pln/courses/d7020e/klee_tutorial/cargo_klee_examples/target/release/examples/cyccnt-012d42640c9faf9d.ll']
// num objects: 3
// object 0: name: 'PRIMASK'
// object 0: size: 4
// object 0: data: b'\x00\x00\x00\x00'
// object 0: hex : 0x00000000
// object 0: int : 0
// object 0: uint: 0
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x00\x00\x00\x00'
// object 1: hex : 0x00000000
// object 1: int : 0
// object 1: uint: 0
// object 1: text: ....
// object 2: name: 'vcell'
// object 2: size: 4
// object 2: data: b'\x00\x00\x00\x00'
// object 2: hex : 0x00000000
// object 2: int : 0
// object 2: uint: 0
// object 2: text: ....
//
// As expected we read the `PRIMASK` and then read `cyccnt` twice.
//
// KLEE does not know which register was read, just that a `vcell`
// was accessed.
//
// We can replay the test:
//
// > cargo klee --example cyccnt --release --replay --gdb
// ...
// Type "apropos word" to search for commands related to "word"...
// Reading symbols from cyccnt.replay...
// (gdb) break main
// Breakpoint 1 at 0x555555555127: file examples/cyccnt.rs, line 11.
// (gdb) set environment KTEST_FILE=klee-last/test000001.ktest
// (gdb) run
// Breakpoint 1, klee_sys::lib_klee_analysis::klee_make_symbolic<u32> (t=0x7fffffffd75c,
//     name=<optimized out>)
//     at /home/pln/.cargo/git/checkouts/klee-sys-7ee2aa8a1a6bbc46/de8fd90/src/lib_klee_analysis.rs:19
// 19              crate::ll::klee_make_symbolic(
//
// Nice!, the very first thing that happens is that we get a symbolic value
// (this is the `PRIMASK`)
//
// (gdb) next
// cortex_m::interrupt::free<closure-0,core::option::Option<cortex_m::peripheral::Peripherals>> (f=...)
//     at /home/pln/.cargo/git/checkouts/cortex-m-514878a7410beb63/07e7543/src/interrupt.rs:75
// 75          let r = f(unsafe { &CriticalSection::new() });
//
// This is the critical section mentioned above:
//
// (gdb) next
// cyccnt::main () at examples/cyccnt.rs:15
// 15          let start: u32 = core.DWT.cyccnt.read();
//
// (gdb) next
// cyccnt::main () at examples/cyccnt.rs:15
// 20          let end = core.DWT.cyccnt.read();
//
// (gdb) next
// 23      }
//
// Pretty much what we could expect.
//
// Now try to `run` the again and print the values of `start`/`end`.
//
// What do you get, and why?
//
// [your answer here]
// $1 = <optimized out> for 'start' and 'No symbol 'end' in current context',
// most likely the compiler did not need them.
//
// As you should have seen, this was not particularly informative, right?
//
// B) In debug/dev mode
//
// Let's replay the example in debug mode?
//
// > cargo klee --example cyccnt
//
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 761
// KLEE: done: completed paths = 4
// KLEE: done: generated tests = 3
//
// WHUT!!!!!
// An error occurred, this was unexpected, right?
//
// Try figure out what caused the error.
//
// Hint, look at the generated tests and open the `testX.abort.err` file.
// (Each failing test comes with a corresponding error file.)
// 
// The error file contains a backtrace.
// Replay the failing test with a breakpoint set to the failing operation.
// Print the values of `start` and `end`, when hitting the breakpoint.
//
// Value of `start`.
//
// [your answer here]
// for start we get$1 = 0.
//
// Value of `end`
//
// [your answer here]
// For end we get $2 = 0.
//
// Why does these values cause an error debug/dev build but not in a release build?
//
// [your answer here]
// The answer will become 0 which is not possible.
//
// C) Fix the problem!
//
// Come up with a solution to the problem, and alter the source code accordingly.
//
// Verify that the program now passes without errors in both debug/dev and
// release builds.
//
// There are numerous ways to solve the problem.
// Argue for your solution in your own words.
//
// [your answer here]
// Make the core execution symbolic similar to what we did in array.rs 
//
// D) Learning outcomes and major takeaways.
//
// In this exercise, you have validated embedded Rust code using KLEE
// and found a typical "hard to find" error.
//
// why are such errors so hard to find?
//
// Think about it?
//
// What is the condition for triggering the error?
//
// What is the behavior of CYCCNT?
//
// Assume the MCU is running at 8MHz.
//
// How long time would lines 16/17 take to run to trigger the error?
//
// [your answer here]
//
// Of course this is a contrived example, and may not occur in practice.
// But, it represents a class of problems/errors/bugs that is
// extremely hard to find by testing.
//
// Ask Jan Van Deventer about the anti-lock breaks he developed
// for Alpha Romeo! (His nick, was Fender Bender at the time.)
//
// Same break controller is present in Opel Speedster, so you can
// ask Johan at Grepit how it feels when you have zero-breaking
// after hitting a bump. (He now bypassed the anti-lock.)
