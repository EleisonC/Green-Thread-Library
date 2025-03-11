.global context_switch
context_switch:
    ;Save callee-saved registers from the 'from' kronocontext
    mov %rbp, 0(%rdi)
    mov %rbx, 8(%rdi)
    mov %r12, 16(%rdi)
    mov %r13, 24(%rdi)
    mov %r14, 32(%rdi)
    mov %r15, 40(%rdi)

    ;Save the stack pointer
    lea 8(%rsp), %rax
    mov %rax, 48(%rdi)

    ;Load callee-saved registers from the 'to' kronocontext
    mov 0(%rsi), %rbp
    mov 8(%rsi), %rbx
    mov 16(%rsi), %r12
    mov 24(%rsi), %r13
    mov 32(%rsi), %r14
    mov 40(%rsi), %r15

    ;Load the stack pointer
    mov 48(%rsi), %rsp

    ;Return to the new context
    ret
