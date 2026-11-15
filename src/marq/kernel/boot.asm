; boot.asm
global _start
extern kernel_main

; Define a dummy StartInfo for the very first boot test
section .data 
dummy_boot_info:
	dq 0x0000000000000000 ; memory_map_addr
	dq 0				  ; memory_map_entries
	dq 0x0000000000000000 ; framebuffer_addr
	dd 0				  ; framebuffer_width
	dd 0 				  ; framebuffer_height

section .text
bits 64

_start:
	; set up the stack
	; reserve 64 kb for the initial boot stack
	mov rsp, stack_top
	
	; align the stack to 16 bytes 
	and rsp, -16
	
	; pass the point to our dummy boot info to the first argument register (rdi)
	mov rdi, dummy_booy_info
	
	; call the rust entry point
	call kernel_main
	
	; if kernel_main ever returns, hang the cpu.
.hang:
	cli
	hlt
	jmp .hang
section .bss 
align 16
stack_bottom:
	resb 65536			; 64 KB stack 
stack_top: