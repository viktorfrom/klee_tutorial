# Klee tutorial

Installing and testing klee.

## Dependencies

Dependencies for building klee is discussed in [building klee](https://klee.github.io/build-llvm9/).

Under the hood, klee uses a `sat` solver for First Order Logic (FOL). Klee can interface to [z3](https://en.wikipedia.org/wiki/Z3_Theorem_Prover) which is modern and efficient solver using Satisfiability Modulo Theories (SMT). In essence SMT is FOL + built in theories for reasoning on fixed-size bit-vectors, extensional arrays, datatypes, and uninterpreted functions, making it suitable for program analysis.

So first install `z3` on your system (then klee will use that instead of the default solver).

Under arch with `yay` installed simply:

```shell
> yay -S z3
```


## Install KLEE from source


The instructions recommend LLVM 9, but the current master (2.2-pre) supports LLVM 11 (which is what you would have under arch as of 2020-12-07).

Under arch you can simply

```shell
> git clone https://github.com/klee/klee.git
> cd klee
> mkdir build
> cd build
> cmake ..
> make -j 8 (-j sets number of parallel builds, e.g., on a 8 threaded machine)
> sudo make install
```

Verify that you have the tool installed.

```shell
> klee -version
KLEE 2.2-pre (https://klee.github.io)
  Build mode: RelWithDebInfo (Asserts: ON)
  Build revision: 199bd43deffc614b2915f4de26475ca43d22e2ae

LLVM (http://llvm.org/):
  LLVM version 11.0.0
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: znver2
```

If your build fails at some point, consult the docs [building klee](https://klee.github.io/build-llvm9/).

## Testing a small function

See the `examples/get_sign` folder.
