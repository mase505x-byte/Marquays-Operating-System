use core::ptr::write_volatile;
	
const COM1_PORT: u16 = 0x3F8;

pub struct SerialPort;

impl SerialPort {
	pub const fn new() -> Self {
		SerialPort 
	}
	
	fn write_byte(&self, byte: u8) {
		unsafe {
			while (read_port(COM1_PORT + 5) & 0x20) == 0 {}
			write_port(COM1_PORT, byte);
		}
	}
	
	pub fn write_str(&self, s: &str) {
		for byte in s.bytes() {
			self.write_byte(byte);
		}
	
	}
}
pub static EARLY_CONSOLE: SerialPort = SerialPort::new();

pub fn init_early() {
	unsafe {
		write_port(COM1_PORT + 1, 0x00);
		write_port(COM1_PORT + 3 0x80);
		write_port(COM1_PORT + 0, 0x03);
		write_port(COM1_PORT + 1, 0x00);
		write_port(COM1_PORT + 3, 0x03);
		write_port(COM1_PORT + 2, 0xC7);
		write_port(COM1_PORT + 4, 0x0B);
	}
	EARLY_CONSOLE,write_str("[Marquays] early console initialized.\n")
}

#[inline(always)]
unsafe fn read_port(port: u16) -> u8 {
	let result: u8;
	core::arch::asm!("in al, dx", out("al") result, in("dx") port);
	result 
}

#[inline(always)]
unsafe fn write_port(port: u16, data: u8) {
	core::arch::asm!("out dx, al", in("dx") port, in("al") data);
}	