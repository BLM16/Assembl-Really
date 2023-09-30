;
; prints: 0123456789
;

mov ebx, 0                  ; ebx holds the loop count
.loop:                      ; loop back here every time
    push ebx
    call asmr::io::print
    
    inc ebx
    cmp ebx, 10
    jne .loop

nop                         ; does nothing

; exit with code 0
mov eax, 0
ret
