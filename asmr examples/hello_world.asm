;
; prints: Hello World!\n
;

hw db "Hello World", 0xa

mov eax, hw
push eax
call asmr::io::print

; exit with code 0
mov eax, 0
ret
