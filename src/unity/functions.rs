use std::ffi::{c_char, c_void};
use super::{types::MethodInfo, module::{MethodPtr, get_method_ptr}};

#[derive(Clone)]
pub struct Il2CppFunctions {
  pub il2cpp_domain_get: Option<MethodPtr<fn() -> *const c_void>>,
  pub il2cpp_domain_get_assemblies: Option<MethodPtr<fn(*const c_void, *const usize) -> *const *const c_void>>,
  pub il2cpp_assembly_get_image: Option<MethodPtr<fn(*const c_void) -> *const c_void>>,
  pub il2cpp_image_get_class_count: Option<MethodPtr<fn(*const c_void) -> usize>>,
  pub il2cpp_image_get_class: Option<MethodPtr<fn(*const c_void, usize) -> *const c_void>>,
  pub il2cpp_class_get_methods: Option<MethodPtr<fn(*const c_void, *const *const c_void) -> *const MethodInfo>>,
  pub il2cpp_class_get_name: Option<MethodPtr<fn(*const c_void) -> *const c_char>>,
  pub il2cpp_class_get_namespace: Option<MethodPtr<fn(*const c_void) -> *const c_char>>,
  pub il2cpp_method_get_name: Option<MethodPtr<fn(*const MethodInfo) -> *const c_char>>
}

impl Il2CppFunctions {
  pub fn new(base: usize) -> Self {
    return Il2CppFunctions {
      il2cpp_domain_get: get_method_ptr(base + 0x1cee5c0),
      il2cpp_domain_get_assemblies: get_method_ptr(base + 0x1cee5d0),
      il2cpp_assembly_get_image: get_method_ptr(base + 0x1cee488),
      il2cpp_image_get_class_count: get_method_ptr(base + 0x1cee8f0),
      il2cpp_image_get_class: get_method_ptr(base + 0x1cee8f8),
      il2cpp_class_get_methods: get_method_ptr(base + 0x1cee4f0),
      il2cpp_class_get_name: get_method_ptr(base + 0x1cee500),
      il2cpp_class_get_namespace: get_method_ptr(base + 0x1cee510),
      il2cpp_method_get_name: get_method_ptr(base + 0x1cee750)
    };
  }
}