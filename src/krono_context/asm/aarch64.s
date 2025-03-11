.global context_switch
context_switch:
	;Save callee-saved registers (x19 to x29)
	;from the 'from' kronocontext (pointed to by x0)
	str x19, [x0, #0]
	str x20, [x0, #8]
	str x21, [x0, #16]
	str x22, [x0, #24]
	str x23, [x0, #32]
	str x24, [x0, #40]
	str x25, [x0, #48]
	str x26, [x0, #56]
	str x27, [x0, #64]
	str x28, [x0, #72]
	str x29, [x0, #80]

	;save the current stack pointer and link register
	mov x9, sp ;Move current SP(stack_pointer) to temporary register x9

	;here is where the extra memory we stored comes in handy
	str x9, [x0, #88] ;Store SP at offset 88 in the context struct

	;return address
	str lr, [x0, #96] ;Store the link register at offset 96


	;Load callee-saved registers from the 'to' kronocontext
	;pointed to by x1
	ldr x19, [x1, #0]
	ldr x20, [x1, #8]
	ldr x21, [x1, #16]
	ldr x22, [x1, #24]
	ldr x23, [x1, #32]
	ldr x24, [x1, #40]
	ldr x25, [x1, #48]
	ldr x26, [x1, #56]
	ldr x27, [x1, #64]
	ldr x28, [x1, #72]
	ldr x29, [x1, #80]

	;load stack pointer and link register from the 'to' context
	ldr x9, [x1, #88]
	move sp, x9
	ldr lr, [x1, #96]

	;Return to the new context
	ret
