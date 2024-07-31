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

## Examples
 - Empty program:
 ```
    ; Implicit exit syscall gets generated at compile time
    ; Therefore you don't need to put explicit exit syscall
 ```
 - Load immediate values:
 ```
    Imm A 123
    Imm B x0f
    Imm C b0001
 ```
 - Arithmetics:
 ```
    Imm A 69
    Imm B 420
    ; Do operations and store the result in C
    Add A B C
    Sub A B C
    Mul A B C
    Div A B C
 ```
 - Stack operations:
 ```
    Imm A 1337
    ; Push the value at A to the stack
    ; The value at A stays there
    Push A
    Imm A 420
    ; Return the original value
    Pop A
 ```
 - Conditional jumping
 ```
    Imm A 21
    Imm B 14
    ; A < B, A > B, ...
    Cmp A B
    ; If 21 != 14 Jump forwards
    Jne 6
    Imm C 123
    Imm C 246
 ```
 - Syscalls and building strings manually
 ```
    ; Couple of defines for readability
    % sys_read 1
    % sys_write 2
    % stdout 0
    % stdin 1
    % buffer_size 6

    ; Save the stack pointer before allocation to D
    Add SP Zero D
    Imm C 54
    Push C
    Imm C 57
    Push C
    Imm C 52
    Push C
    Imm C 50
    Push C
    Imm C 48
    Push C
    Imm C 10
    Push C

    ; Push the first address of the string into the stack
    Push D

    Imm A sys_write
    Imm B stdout
    ; Acquire the first addr of the string from the stack
    Pop C
    Imm D buffer_size
    Syscall
 ```
 - For loop :D
 ```
    ; for (int i = 10; i > 0; i--)
    Imm A 10
    Imm B 1
    Cmp A A
    ; Jump out of loop if counter reached 0
    Jz 7
    ; Decrement counter
    Sub A B A
    ; Jump back to begining of loop
    Jmp 2
    Imm D 69
 ```

## Todo
 - Labels
