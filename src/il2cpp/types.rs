use std::ffi::c_void;

#[repr(C)]
pub struct MethodInfo {
  pub invoker_method: *const c_void,
  pub method_pointer: *const c_void,
  _pad: [u8; 0x20],
  pub flags: u16
}