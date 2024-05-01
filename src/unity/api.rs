use std::{ffi::{c_char, c_void, CStr}, path::PathBuf};
use super::{functions::Il2CppFunctions, module::Module, types::MethodInfo};

pub struct Il2Cpp {
  pub unity_player: Module,
  pub game_assembly: Module,
  pub functions: Il2CppFunctions
}

impl Il2Cpp {
  pub fn new(path: PathBuf) -> Self {
    let unity_player = Module::load(path.join("UnityPlayer.dll"));
    let game_assembly = Module::load(path.join("GameAssembly.dll"));
    let functions = Il2CppFunctions::new(unity_player.handle as usize);

    return Il2Cpp {
      unity_player: unity_player,
      game_assembly: game_assembly,
      functions: functions
    };
  }

  fn to_string(native: *const c_char) -> String {
    let str = unsafe { CStr::from_ptr(native) };
    return str.to_str().unwrap().to_string();
  }

  pub fn domain_get(&self) -> *const c_void {
    let function = &self.functions.clone().il2cpp_domain_get.unwrap();
    return function();
  }

  pub fn domain_get_assemblies(&self, domain: *const c_void, size: *const usize) -> *const *const c_void {
    let function = &self.functions.clone().il2cpp_domain_get_assemblies.unwrap();
    return function(domain, size);
  }

  pub fn assembly_get_image(&self, assembly: *const c_void) -> *const c_void {
    let function = &self.functions.clone().il2cpp_assembly_get_image.unwrap();
    return function(assembly);
  }

  pub fn image_get_class_count(&self, image: *const c_void) -> usize {
    let function = &self.functions.clone().il2cpp_image_get_class_count.unwrap();
    return function(image);
  }

  pub fn image_get_class(&self, image: *const c_void, index: usize) -> *const c_void {
    let function = &self.functions.clone().il2cpp_image_get_class.unwrap();
    return function(image, index);
  }

  pub fn class_get_methods(&self, class: *const c_void, iter: *const *const c_void) -> Option<*const MethodInfo> {
    let function = &self.functions.clone().il2cpp_class_get_methods.unwrap();
    let result = function(class, iter);
    return if !result.is_null() { Some(result) } else { None };
  }

  pub fn class_get_name(&self, class: *const c_void) -> String {
    let function = &self.functions.clone().il2cpp_class_get_name.unwrap();
    return Self::to_string(function(class));
  }

  pub fn class_get_namespace(&self, class: *const c_void) -> String {
    let function = &self.functions.clone().il2cpp_class_get_namespace.unwrap();
    return Self::to_string(function(class));
  }

  pub fn method_get_name(&self, method: *const MethodInfo) -> String {
    let function = &self.functions.clone().il2cpp_method_get_name.unwrap();
    return Self::to_string(function(method));
  }
}

pub static mut RUNTIME: Option<Il2Cpp> = None;

pub fn get_il2cpp_api() -> &'static Il2Cpp {
  unsafe {
    if RUNTIME.is_none() {
      let base_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
      RUNTIME = Some(Il2Cpp::new(base_path));
    }
  
    return RUNTIME.as_ref().unwrap();
  }
}