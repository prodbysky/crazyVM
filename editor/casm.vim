if exists("b:current_syntax")
    finish
endif

syn match casmLiteral display "[0-9]"
syn match casmLiteral display "#[0-f]"
syn match casmLiteral display "$[0-1]"
syn match casmComment display ";*."

syn keyword casmKeyword
 \ Add
 \ Sub
 \ Mul
 \ Div
 \ Imm
 \ Push
 \ Pop
 \ StackAdd
 \ StackSub
 \ StackMul
 \ StackDiv
 \ Cmp
 \ Jmp
 \ Je
 \ Jne
 \ Jg
 \ Jge
 \ Jl
 \ Jle
 \ Jz
 \ Jnz
 \ Ret
 \ Call
 \ Fn
 \ Syscall

syn keyword casmRegister
 \ A
 \ B
 \ C
 \ D
 \ SP
 \ PC
 \ Zero

hi def link casmLiteral Number
hi def link casmRegister Type
hi def link casmKeyword Function

let b:current_syntax = "casm"
