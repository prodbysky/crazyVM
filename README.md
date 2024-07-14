# CrazyVM
## 32bit virtual machine written in Rust!

This is my first attempt at writing a VM, and 
an assembly language, assembler, and in the future
a programming language with a compiler! The inspiration for
this project came from Tsodings [BM](https://github.com/tsoding/bm), and Tom Marks' [rust-simple-vm](https://github.com/phy1um/rust-simple-vm)

## Features of this project
 - A simple VM that interprets the instructions [below](#instructions)
 - Emulated CPU architecture

## Instructions
- Add:
    10000000 xxx yyy zzz 0000000 00000000
             r 1 r 2 r 3

- Immediate value
    11100000 xxxy yyyy yyyy yyyy
             r 1  literal

-  Push to stack
    00010000 xxx 00000 00000000
             r 1
