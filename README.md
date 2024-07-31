# CrazyVM
## 32bit virtual machine written in Rust!

This is my first attempt at writing a VM, and 
an assembly language, assembler, and in the future
a programming language with a compiler! The inspiration for
this project came from Tsodings [BM](https://github.com/tsoding/bm), and Tom Marks' [rust-simple-vm](https://github.com/phy1um/rust-simple-vm)

## Features of this project
 - A simple VM that interprets the crazyVM bytecode
 - A simple assembly language
 - Emulated CPU architecture

## Syscalls
Syscall number goes in the A register
 - sys_exit  (0):
  - B register: exit_code (uint)

 - sys_read  (1):
  - B register: file descriptor (uint)
  - C register: base buffer pointer (ptr)
  - D register: buffer length (uint)
 - sys_write (2):
  - B register: file descriptor (uint)
  - C register: base buffer pointer (ptr)
  - D register: buffer length (uint)

## Todo
 - Labels
