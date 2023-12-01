section .text

global _start

; rdi: int
print_num:
    push rbp
    mov rbp, rsp

    ; init the buffer with linefeeds
    ; 8 bytes should be plenty
    mov rax, 0x0A0A0A0A0A0A0A0A
    mov qword [rbp], rax
    mov dword [rbp+0x10], 0
    mov rsi, rbp
    ; format the number as ascii
.loop:
    cmp rdi, 0
    je .done
    dec rsi
    ; div/mod rdi by 10
    mov rax, rdi
    mov rdx, 0
    mov rbx, 0xA
    div rbx
    add rdx, 0x30 ; remainder+'0'
    mov byte [rsi], dl
    mov rdi, rax
    inc dword [rbp+0x10]
    jmp .loop
.done:
    ; rsi points to the buffer
    mov edx, dword [rbp+0x10] ;   len
    inc rdx                   ; account for the linefeed at the end
    ; write linefeed after the end
    lea rax, [rsi + rdx]
    mov byte [rax], 0xA
    ; write
    mov rax, 1                ;   write
    mov rdi, 1                ;   STDOUT
    syscall
    pop rbp
    ret


; rdi: filedescriptor
; read the next line and return the number of the line
process_line_v1:
  push rbp
  mov rbp, rsp
  ; use the red zone for locals
  mov dword [rbp-0x20], 0x0 ; first
  mov dword [rbp-0x24], 0x0 ; last
  mov dword [rbp-0x2C], edi
.loop:
  ; read the next byte
  mov eax, 0
  mov edi, dword[rbp-0x2C]
  lea rsi, [rbp-0x30]
  mov rdx, 1
  syscall
  ; load the next char into rbx
  xor rbx, rbx
  mov bl, [rbp-0x30]
  ; test '\n' == rbx
  cmp rbx, 0xA ; '\n'
  je .done
  ; test '0' <= rbx <= '9'
  cmp rbx, 0x30 ; '0'
  jl .loop
  cmp rbx, 0x39 ; '9'
  ja .loop
  ; rbx is a digit
  sub rbx, 0x30
  mov dword [rbp-0x24], ebx
  ; if first is set, then do not set again
  cmp dword [rbp-0x20], 0x0
  jnz .loop
  mov dword [rbp-0x20], ebx
  jmp .loop
.done:
  mov rax, 0xa
  mul dword [rbp-0x20]
  add eax, dword [rbp-0x24]
  pop rbp
  ret

_start:
  mov rax, 1
  pop rbx               ; argc
  cmp rbx, 1
  je .done
  pop rbx               ; argv[0]
  pop rbx               ; filename

  ; open the file
  mov eax, 2
  mov rdi, rbx
  mov rsi, 0
  mov rdx, 0666
  syscall
  push rax
  push 0

.loop:
  mov rdi, qword [rsp+8]
  call process_line_v1
  cmp rax, 0
  je .done
  add qword [rsp], rax
  jmp .loop
.done:
  mov rdi, qword [rsp]
  call print_num

  ; exit
  mov rax, 60
  mov rdi, 0
  syscall
