prompt db   "What is your name: "
msg    db   "Welcome, "
exmark db   "!"
name   resb 50

; prompt the user for their name
push prompt
call asmr::io::println

; read the name
; readln will save the value to the last stack value's address
; readln pops this buffer off the stack leaving the value in the buffer
push name
call asmr::io::readln

; push values to print in reverse order
; all asmr:: calls pop their arguments from the stack
push 0xa
push exmark
push name
push msg

; call println 4 times to print each value off the stack
; println pops the last value off of the stack each call
mov edx, 4
.loop:
    call asmr::io::println
    dec edx
    cmp edx, 0
    jne .loop

; exit
mov eax, 0
ret
