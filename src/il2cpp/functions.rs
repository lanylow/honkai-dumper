use std::ffi::{c_char, c_void};

use super::{types::*, module::FunctionPtr};

macro_rules! index {
  ($start:expr, $index:expr) => {{
    let addr = unsafe { *$start.offset($index) };
    if addr.is_null() { None } else { Some(FunctionPtr::new(addr)) }
  }};
}

#[derive(Clone)]
pub struct Il2CppFunctions {
  pub il2cpp_assembly_get_image: Option<FunctionPtr<fn(*const Il2CppAssembly) -> *const Il2CppImage>>,

  pub il2cpp_class_get_fields: Option<FunctionPtr<fn(*const Il2CppClass, *const *const c_void) -> *const FieldInfo>>,
  pub il2cpp_class_get_interfaces: Option<FunctionPtr<fn(*const Il2CppClass, *const *const c_void) -> *const Il2CppClass>>,
  pub il2cpp_class_get_methods: Option<FunctionPtr<fn(*const Il2CppClass, *const *const c_void) -> *const MethodInfo>>,
  pub il2cpp_class_get_name: Option<FunctionPtr<fn(*const Il2CppClass) -> *const c_char>>,
  pub il2cpp_class_get_namespace: Option<FunctionPtr<fn(*const Il2CppClass) -> *const c_char>>,
  pub il2cpp_class_get_parent: Option<FunctionPtr<fn(*const Il2CppClass) -> *const Il2CppClass>>,
  pub il2cpp_class_is_valuetype: Option<FunctionPtr<fn(*const Il2CppClass) -> bool>>,
  pub il2cpp_class_get_flags: Option<FunctionPtr<fn(*const Il2CppClass) -> i32>>,
  pub il2cpp_class_from_type: Option<FunctionPtr<fn(*const Il2CppType) -> *const Il2CppClass>>,
  pub il2cpp_class_is_enum: Option<FunctionPtr<fn(*const Il2CppClass) -> bool>>,

  pub il2cpp_domain_get: Option<FunctionPtr<fn() -> *const Il2CppDomain>>,
  pub il2cpp_domain_get_assemblies: Option<FunctionPtr<fn(*const Il2CppDomain, *const usize) -> *const *const Il2CppAssembly>>,
  
  pub il2cpp_field_get_flags: Option<FunctionPtr<fn(*const FieldInfo) -> i32>>,
  pub il2cpp_field_get_name: Option<FunctionPtr<fn(*const FieldInfo) -> *const c_char>>,
  pub il2cpp_field_get_offset: Option<FunctionPtr<fn(*const FieldInfo) -> usize>>,
  pub il2cpp_field_get_type: Option<FunctionPtr<fn(*const FieldInfo) -> *const Il2CppType>>,

  pub il2cpp_method_get_return_type: Option<FunctionPtr<fn(*const MethodInfo) -> *const Il2CppType>>,
  pub il2cpp_method_get_name: Option<FunctionPtr<fn(*const MethodInfo) -> *const c_char>>,
  pub il2cpp_method_get_param_count: Option<FunctionPtr<fn(*const MethodInfo) -> u32>>,
  pub il2cpp_method_get_param: Option<FunctionPtr<fn(*const MethodInfo, u32) -> *const Il2CppType>>,

  pub il2cpp_type_get_name: Option<FunctionPtr<fn(*const Il2CppType) -> *const c_char>>,
  pub il2cpp_type_is_byref: Option<FunctionPtr<fn(*const Il2CppType) -> bool>>,
  pub il2cpp_type_get_attrs: Option<FunctionPtr<fn(*const Il2CppType) -> u32>>,

  pub il2cpp_image_get_name: Option<FunctionPtr<fn(*const Il2CppImage) -> *const c_char>>,
  pub il2cpp_image_get_class_count: Option<FunctionPtr<fn(*const Il2CppImage) -> usize>>,
  pub il2cpp_image_get_class: Option<FunctionPtr<fn(*const Il2CppImage, usize) -> *const Il2CppClass>>
}

impl Il2CppFunctions {
  pub fn new(base: usize) -> Self {
    let funcs = (base + 0x1ed4ee8) as *const *const c_void;

    Il2CppFunctions {
      // Required for the method dumper to work
      il2cpp_assembly_get_image: index!(funcs, 22),
      il2cpp_class_get_methods: index!(funcs, 35),
      il2cpp_class_get_name: index!(funcs, 37),
      il2cpp_class_get_namespace: index!(funcs, 39),
      il2cpp_domain_get: index!(funcs, 63),
      il2cpp_domain_get_assemblies: index!(funcs, 65),
      il2cpp_method_get_name: index!(funcs, 117),
      il2cpp_image_get_class_count: index!(funcs, 169),
      il2cpp_image_get_class: index!(funcs, 170),

      // Optional for the C# dumper
      il2cpp_class_get_fields: index!(funcs, 31),
      il2cpp_class_get_interfaces: index!(funcs, 33),
      il2cpp_class_get_parent: index!(funcs, 40),
      il2cpp_class_is_valuetype: index!(funcs, 43),
      il2cpp_class_get_flags: index!(funcs, 45),
      il2cpp_class_from_type: index!(funcs, 49),
      il2cpp_class_is_enum: index!(funcs, 53), 
      il2cpp_field_get_flags: index!(funcs, 72),
      il2cpp_field_get_name: index!(funcs, 73),
      il2cpp_field_get_offset: index!(funcs, 75),
      il2cpp_field_get_type: index!(funcs, 76),
      il2cpp_method_get_return_type: index!(funcs, 116),
      il2cpp_method_get_param_count: index!(funcs, 123),
      il2cpp_method_get_param: index!(funcs, 124),
      il2cpp_type_get_name: index!(funcs, 161),
      il2cpp_type_is_byref: index!(funcs, 162),
      il2cpp_type_get_attrs: index!(funcs, 163),
      il2cpp_image_get_name: index!(funcs, 168)
    }
  }
}
