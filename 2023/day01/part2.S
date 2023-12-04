section .text

global _start

; rdi: str haystack, null terminated
; rsi: str needle
; rdx: len
startswith:
.loop:
    cmp rdx, 0
    je .done
    ; TODO: maybe do not do this 1 byte at a time
    mov bl, byte [rdi]
    cmp bl, 0
    je .fail
    mov cl, byte [rsi]
    cmp bl, cl
    jne .fail
    dec rdx
    inc rdi
    inc rsi
    jmp .loop
.done:
    mov eax, 1
    ret
.fail:
    mov eax, 0
    ret

; rdi: int
print_num:
    push rbp
    mov rbp, rsp

    ; init the buffer with linefeeds
    ; 8 bytes should be plenty
    mov rax, 0x0A0A0A0A0A0A0A0A
    mov qword [rbp], rax
    mov dword [rbp-0x10], 0
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
    inc dword [rbp-0x10]
    jmp .loop
.done:
    ; rsi points to the buffer
    mov edx, dword [rbp-0x10] ;   len
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


; rdi: string
; rsi: out string
; read the next line and return the number of the line
process_line:
  push rbp
  push rsi
  mov rbp, rsp
  ; use the red zone for locals
  mov dword [rbp-0x20], 0x0 ; first
  mov dword [rbp-0x24], 0x0 ; last
  mov qword [rbp-0x30], rdi
  dec qword [rbp-0x30]
.loop:
  ; load the next char into rbx
  inc qword [rbp-0x30]
  mov rax, qword [rbp-0x30]
  xor rbx, rbx
  mov bl, byte [rax]
  ; test '\n' == rbx || rbx == '\0'
  cmp rbx, 0xA ; '\n'
  je .done
  cmp rbx, 0x0
  je .done
  ; test '0' <= rbx <= '9'
  cmp rbx, 0x30 ; '0'
  jl .compare_words
  cmp rbx, 0x39 ; '9'
  ja .compare_words
  ; rbx is a digit
  sub rbx, 0x30
  jmp .add
.compare_words:
; one
  mov rdi, [rbp-0x30]
  mov rsi, one
  mov rdx, one_len
  call startswith
  mov rbx, 1
  cmp eax, 1
  je .add
; two
  mov rdi, [rbp-0x30]
  mov rsi, two
  mov rdx, two_len
  call startswith
  mov rbx, 2
  cmp eax, 1
  je .add
; three
  mov rdi, [rbp-0x30]
  mov rsi, three
  mov rdx, three_len
  call startswith
  mov rbx, 3
  cmp eax, 1
  je .add
; four
  mov rdi, [rbp-0x30]
  mov rsi, four
  mov rdx, four_len
  call startswith
  mov rbx, 4
  cmp eax, 1
  je .add
; five
  mov rdi, [rbp-0x30]
  mov rsi, five
  mov rdx, five_len
  call startswith
  mov rbx, 5
  cmp eax, 1
  je .add
; six
  mov rdi, [rbp-0x30]
  mov rsi, six
  mov rdx, six_len
  call startswith
  mov rbx, 6
  cmp eax, 1
  je .add
; seven
  mov rdi, [rbp-0x30]
  mov rsi, seven
  mov rdx, seven_len
  call startswith
  mov rbx, 7
  cmp eax, 1
  je .add
; eight
  mov rdi, [rbp-0x30]
  mov rsi, eight
  mov rdx, eight_len
  call startswith
  mov rbx, 8
  cmp eax, 1
  je .add
; nine
  mov rdi, [rbp-0x30]
  mov rsi, nine
  mov rdx, nine_len
  call startswith
  mov rbx, 9
  cmp eax, 1
  je .add
  jmp .loop
.add:
  mov dword [rbp-0x24], ebx
  ; if first is set, then do not set again
  cmp dword [rbp-0x20], 0x0
  jne .loop
  mov dword [rbp-0x20], ebx
  jmp .loop
.done:
  ; 10 * rbp-0x20 + rbp-0x24
  mov rax, 0xA
  mul dword [rbp-0x20]
  add eax, dword [rbp-0x24]
  ; return the current ptr
  mov rbx, qword [rbp-0x30]
  pop rsi
  inc rbx ; point 1 after the last char
  mov [rsi], rbx
  pop rbp
  ret

_start:
  mov rax, 1
  pop rbx               ; argc
  cmp rbx, 1
  je .done
  pop rbx               ; argv[0]
  pop rbx               ; filename

  sub rsp, 144

  ; open the file
  mov eax, 2
  mov rdi, rbx
  mov rsi, 0
  mov rdx, 0666
  syscall
  push rax

  ; stat the file to obtain its length
  mov rdi, rax ; fd
  mov eax, 5
  lea rsi, [rsp+8]
  syscall

  ; memory map the file
  mov eax, 9
  xor rdi, rdi
  mov rsi, [rsp+48+8]
  mov rdx, 0x1 ; PROT_READ
  mov r10, 0x2 ; MAP_PRIVATE
  mov r8, qword [rsp] ; fd
  mov r9, 0
  syscall

  push qword 0 ; out ptr for process_line
  push qword 0 ; total

  mov rdi, rax
.loop:
  lea rsi, [rsp+8]
  call process_line
  cmp rax, 0
  je .done
  add qword [rsp], rax
  ; mov rdi, rax
  ; call print_num
  mov rdi, [rsp+8]
  jmp .loop
.done:
  mov rdi, qword [rsp]
  call print_num

  add rsp, 160

  ; exit
  mov rax, 60
  mov rdi, 0
  syscall

section .rodata
one db "one"
one_len equ $ -one

two db "two"
two_len equ $ -two

three db "three"
three_len equ $ -three

four db "four"
four_len equ $ -four

five db "five"
five_len equ $ -five

six db "six"
six_len equ $ -six

seven db "seven"
seven_len equ $ -seven

eight db "eight"
eight_len equ $ -eight

nine db "nine"
nine_len equ $ -nine
