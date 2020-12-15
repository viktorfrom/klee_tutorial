/*
 * First KLEE tutorial: testing a small function
 * https://klee.github.io/tutorials/testing-function/
 */

#include <klee/klee.h>

int get_sign(int x)
{
    if (x == 0)
        return 0;

    if (x < 0)
        return -1;
    else
        return 1;
}

int main()
{
    int a;
    klee_make_symbolic(&a, sizeof(a), "a");
    return get_sign(a);
}

// A) Compiling into LLVM bitcode
// > clang -emit-llvm -c get_sign.c
//
// Now you can run Klee on your generated bitcode.
//
// > klee get_sign.bc
//
// KLEE: done: total instructions = 31
// KLEE: done: completed paths = 3
// KLEE: done: generated tests = 3
//
// B) Inspecting the output
//
// > ls klee-last/
//
// [your answer here]
//  assembly.ll  info  messages.txt  run.istats  
//  run.stats	test000001.ktest  test000002.ktest  
//  test000003.ktest  warnings.txt
//
// C) Inspecting the generated test cases
//
// > ktest-tool klee-last/test000001.ktest
//
// What path in the code does this test represent?
//
// [your answer here]
// The first if-statement, if (x == 0) return 0;
//
// > ktest-tool klee-last/test000002.ktest
//
// What path in the code does this test represent?
//
// [your answer here]
// The third if-statement, else return 1;
// 
// > ktest-tool klee-last/test000003.ktest
//
// What path in the code does this test represent?
//
// [your answer here]
// The second if-statement, if (x < 0) return -1;
//
// D) Replaying a test case
//
// First check that includes were installed:
// > ls /usr/local/include
// klee
//
// > ls /usr/local/lib
// klee  libkleeRuntest.so  libkleeRuntest.so.1.0
//
// If you installed Klee using the package manager
// the path might be different:
//
// Using `aur` (arch) files are stored in the system default
// folders, `/usr/include` and `/usr/lib`.
//
// If those are ok, then you can compile for replay:
//
// > clang -I /usr/local/include/ -L /usr/local/lib get_sign.c -l kleeRuntest
//
// Or just
// > clang get_sign -l kleeRuntest
//
// If the `include` and `lib` paths are the system defaults.
//
// To replay the first test:
//
// We need to add the libary path so it can be dynamically loaded:
// Depending on shell this might look different:
//
// Under `bash` (and `bash` like shells)
// > export LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH
//
// Under `fish`
// > set -x LD_LIBRARY_PATH /usr/local/lib/:$LD_LIBRARY_PATH
//
// Once again, if using the system default system folders
// you don't need to add anything to `LD_LIBRARY_PATH`.
//
// > KTEST_FILE=klee-last/test000001.ktest ./a.out
//
// Now let's inspect the status (return code), in `bash`:
// $? is the return value (error code) as seen by the shell.
//
// > echo $?
//
// In `fish` you would do
//
// > echo $status
//
// Did the result correspond to the expected path for the test?
//
// [your answer here]
// "0" was returned which was the expected path. (first)
//
// > KTEST_FILE=klee-last/test000002.ktest ./a.out
//
// Inspect the return code:
//
// Did the result correspond to the expected path for the test?
//
// [your answer here]
// "1" was returned which was the expected path. (third)
//
// > KTEST_FILE=klee-last/test000003.ktest ./a.out
//
// Inspect the return code:
//
// Did the result correspond to the expected path for the test?
//
// [your answer here]
// "255" was returned. (second)
//
// Why not? Confer to shell error codes:
//
// [your answer here]
// Since 255 was returned for -1 means we got an out of range answer. 
//
// D) Debugging
//
// In the above example its kind of hard to see exactly
// what happens. Using `gdb` you single step the program.
//
// First build it with debug symbols (`-g`).
// > clang -g -I /usr/local/include/ -L /usr/local/lib get_sign.c -l kleeRuntest
//
// Or if using system defaults:
// > clang -g get_sign.c -l kleeRuntest
//
// Then start `gdb`:
// > KTEST_FILE=klee-last/test000001.ktest gdb ./a.out
// (gdb) break get_sign
//
// (gdb) run
//
// Now we can inspect the `x` argument by:
// (gdb) print x
//
// What value do you get, and why?
//
// [your answer here]
// $1 = 0
//
// Step the code
// > (gdb) next
//
// What path did it take, and why?
//
// [your answer here]
// return 0; because x = 0.
//
// Now we can try with another test:
//
// (gdb) set environment KTEST_FILE=klee-last/test000002.ktest
//
// And (re-start) the debug session:
// (gdb) run
//
// Step through the code.
//
// Which path did it take, and why?
//
// [your answer here]
// $1 = 255, third path because x = 255.
//
// And finally:
//
// (gdb) set environment KTEST_FILE=klee-last/test000003.ktest
//
// Which path did it take, and why?
//
// [your answer here]
// $1 = -2147483648, second path because x = -2147483648.
//
// E) Under the hood.
//
// Explain in your own words how
// `klee_make_symbolic(&a, sizeof(a), "a");`
// works when you run `klee` to generate test cases:
//
// [your answer here]
// (hint, mark memory region as symbolic)
// "a" is marked as a symbolic value in Klee and is 
// then allowed to determine valid paths in the program. 
//
// Explain in your own words how
// `klee_make_symbolic(&a, sizeof(a), "a");`
// works when you replay test cases:
//
// [your answer here]
// (hint, KTEST_FILE points to a concrete assignment
// of the memory region)
// "a" is replaces by the values of the test cases.
