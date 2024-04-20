.type _start, @function
.global _start
_start:
    # Pass argc/argv to main
    pop	rdi
    mov rsi, rsp
    push rdi
    # This doesn't return
    call __cloyster_start
