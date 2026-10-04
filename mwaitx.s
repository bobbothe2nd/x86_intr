	.text
	.intel_syntax noprefix
	.file	"mwaitx.c"
	.globl	rust_mwaitx                     # -- Begin function rust_mwaitx
	.p2align	4, 0x90
	.type	rust_mwaitx,@function
rust_mwaitx:                            # @rust_mwaitx
	.cfi_startproc
# %bb.0:
	push	rbx
	.cfi_def_cfa_offset 16
	.cfi_offset rbx, -16
	mov	ebx, edx
	mov	eax, esi
	mov	ecx, edi
	#APP

	mwaitx

	#NO_APP
	pop	rbx
	.cfi_def_cfa_offset 8
	ret
.Lfunc_end0:
	.size	rust_mwaitx, .Lfunc_end0-rust_mwaitx
	.cfi_endproc
                                        # -- End function
	.ident	"Debian clang version 14.0.6"
	.section	".note.GNU-stack","",@progbits
	.addrsig
