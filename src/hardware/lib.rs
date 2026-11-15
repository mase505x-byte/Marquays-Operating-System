#![no_std]

pub mod boot;
pub mod console;
pub mod reconfiguration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hardware Error {
  Unsupported,
  InvlaidImage,
  Busy, 
  Timeout, 
  VerificationFailed, 
  Io,
}
