hw db "Hello World", 0xa

push hw
call asmr::io::println

mov eax, 0
ret
