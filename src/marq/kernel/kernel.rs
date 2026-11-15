#![no_std]

use crate::marq::kernel;
use crate::marq::kernel::core_local::{core_id, core_scheduler};
use crate::marq::kernel::interupts;
use crate::env::{self, StartInfo{;
use crate:{console, drivers, executor, fs, logging, mm, syscalls};

pub mod built_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn kernel_main() {
	built_info::init_target_hardware();
}