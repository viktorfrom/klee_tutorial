# KLEE tutorial

Installing and testing klee.

## Dependencies

Dependencies for building klee is discussed in [building klee](https://klee.github.io/build-llvm9/).

Under the hood, klee uses a `sat` solver for First Order Logic (FOL). Klee can interface to [z3](https://en.wikipedia.org/wiki/Z3_Theorem_Prover) which is modern and efficient solver using Satisfiability Modulo Theories (SMT). In essence SMT is FOL + built in theories for reasoning on fixed-size bit-vectors, extensional arrays, datatypes, and uninterpreted functions, making it suitable for program analysis.

So first install `z3` on your system (then klee will use that instead of the default solver).

Later, you also need to have `gdb` installed, under arch by:

### Arch linux:

Under arch with `yay` installed simply:

```shell
> yay -S z3
```

```shell
> yay -S gdb
```

### Ubuntu (like) systems

> sudo apt install z3 libz3-4 libz3-cil libz3-dev libz3-java libz3-jni libz3-ocaml-de


## Install KLEE from source

The instructions recommend LLVM 9, but the current master (2.2-pre) supports LLVM 11 (which is what you would have under arch as of 2020-12-07).

Under arch you can simply:

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

## Install KLEE from `aur` (arch linux)

The `aur` package [klee](https://aur.archlinux.org/packages/klee/), installs KLEE in `/usr/bin` (binaries), `/usr/include` (C-include files), and `usr/lib` (libraries). These are the default system folders respectively, so it makes it easier to compile, link, and run the KLEE tools.

```shell
> yay -S klee
> klee -version
KLEE 2.2 (https://klee.github.io)
  Build mode: Release (Asserts: ON)
  Build revision: 5719d2803e93252e5d4613f43afc7db0d72332f1

LLVM (http://llvm.org/):
  LLVM version 11.0.0
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: skylake
```

Notice, if you have previously installed from source, but want to use the `aur` instead you should remove the source installed files found in `/usr/local/bin`, `/usr/local/include` and `/usr/local/lib` (please make sure that you only remove the KLEE related files).

---

## Testing a small function

See the `examples/get_sign` folder.
