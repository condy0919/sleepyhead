#ifndef __x86_64__
#error "Wrong architecture: x86-64 expected"
#endif

// fn context_init(ctx: *mut Context, entrance: EntranceFn, arg: *mut u8, stack_base: *mut u8)
.text
.global context_init
.type context_init,@function
.align 16
context_init:
    movq %rsi, 128(%rdi) // ctx->rip = entrance
    movq %rcx, 72(%rdi)  // ctx->rsp = stack_base
    movq %rdx, (%rdi)    // ctx->rdi = arg
    // Does it need a entrance at the bottom of stack_base?
    ret



// fn context_switch(to: *mut Context, from: *mut Context)
.text
.global context_switch
.type context_switch,@function
.align 16
context_switch:
    // save
    movq %rdi, (%rsi)
    movq %rsi, 8(%rsi)
    movq %rdx, 16(%rsi)
    movq %rcx, 24(%rsi)
    movq %r8, 32(%rsi)
    movq %r9, 40(%rsi)
    movq %rax, 48(%rsi)
    movq %rbx, 56(%rsi)
    movq %r10, 64(%rsi)
    movq %rsp, 72(%rsi) // Shoule it put at the enf of this phrase?
    movq %rbp, 80(%rsi)
    movq %r11, 88(%rsi)
    movq %r12, 96(%rsi)
    movq %r13, 104(%rsi)
    movq %r14, 112(%rsi)
    movq %r15, 120(%rsi)

    // restore
    movq 8(%rdi), %rsi
    movq 16(%rdi), %rdx
    movq 24(%rdi), %rcx
    movq 32(%rdi), %r8
    movq 40(%rdi), %r9
    movq 48(%rdi), %rax
    movq 56(%rdi), %rbx
    movq 64(%rdi), %r10
    movq 72(%rdi), %rsp
    movq 80(%rdi), %rbp
    movq 88(%rdi), %r11
    movq 96(%rdi), %r12
    movq 104(%rdi), %r13
    movq 112(%rdi), %r14
    movq 120(%rdi), %r15
    movq (%rdi), %rdi

    ret
