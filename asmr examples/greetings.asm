prompt db   "What is your name: "
msg    db   "Welcome, "
exmark db   "!"
newln  db   0xa
name   resb 50

; prompt the user for their name
mov eax, prompt
push eax
call asmr::io::print

; read the name into the reserved buffer `name`
mov eax, name
push eax
call asmr::io::readln

; push values to print in reverse order
; all asmr:: calls pop their arguments from the stack
mov eax, newln
mov ebx, exmark
mov ecx, name
mov edx, msg
push eax, ebx, ecx, edx

; call println 4 times to print each value off the stack
; println pops the last value off of the stack each call
mov edx, 4
.loop:
    call asmr::io::print
    dec edx
    cmp edx, 0
    jne .loop

; exit with code 0
mov eax, 0
ret
