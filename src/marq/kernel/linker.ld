/* linker.ld */ 
ENTRY(_start_

SECTIONS {
	/* Start at 1 MiB, a safe location for kernels to be loaded */ 
	. = 1M;
	
	.boot : {
		*(.boot)
	}
	
	.text: {
		*(.text .text.*)
	}
	
	.rodata : {
		*(.rodata .rodata.*)
	}
	
	.data : {
		*(.data .data.*)
	}
	
	.bss : {
		*(.bss .bss.*)
	
	/* discard unnecessary sections */
	/DISCARD. : {
		*(.eh_frame)
		*(.comment)
	}
}