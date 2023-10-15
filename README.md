# Concolic Executor

## Overview

This repository contains:
1. Concolic executor written with `RUST` + `Z3`
2. Concolic executor written with `LLVM IR`


This program attempts to solve:
- `x^2 + y^2 = z^2`
- Print out the result

## Requirement

1. Install LLVM
  - On Mac:
    - `brew install llvm`
2. Install Z3

## Instructions for `1`:
1. Compile the LLVM IR to an Object File
- You need to compile your LLVM IR code into an object file using the LLVM llc (LLVM Static Compiler) utility. The object file contains machine code instructions that can be executed.

   - `/opt/homebrew/opt/llvm@17/bin/llc -filetype=obj concolic_executor.ll`

2. Link the Object File
- After generating the object file, you may need to link it with any necessary libraries to create an executable. Use the clang or gcc compiler to link the object file.

   - `clang -o concolic_executor concolic_executor.o`

3. Run the Executable

   - `./concolic_executor`


## Instructions for `2`

1. Move to directory `<Fuzzing>/src`

2. `cargo build --release`

3. `cargo run`
