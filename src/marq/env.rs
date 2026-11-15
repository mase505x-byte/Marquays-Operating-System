// src/env.rs

#[repr(C)]
pub struct StartInfo {
	pub memory_map_addr: usize,
	pub memory_map_entires: usize,
	pub framebuffer_addr: usize,
	pub framebuffer_width: u32,
	pub framebuffer_height: u32,
	// other boot params (e.g., ACPI RSDP pointer, initrd location) 
}
