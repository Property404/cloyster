.type _start, @function
.global _start
_start:
    # Pass argc/argv to main
    lw a0, 0(sp)
    mv a1, sp
    addi a1, a1, 8
    # This doesn't return
    call __cloyster_start
