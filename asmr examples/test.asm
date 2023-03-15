mov eax, 0xa
push eax
call asmr::io::println

nop

; loop 10 times and print the number
mov ebx, 0
.loop:                      ; loop back here every time
    push ebx
    call asmr::io::println
    inc ebx
    cmp ebx, 10
    jne .loop

mov eax, 0
ret
