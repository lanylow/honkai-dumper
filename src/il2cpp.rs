use std::{ffi::{c_char, c_void, CStr, CString}, marker::PhantomData, ops::Deref, path::PathBuf};
use winapi::um::libloaderapi::LoadLibraryA;

#[derive(Clone)]
pub struct MethodPtr<T> {
  pub ptr: *mut c_void,
  pd: PhantomData<T>
}

impl<T> Deref for MethodPtr<T> {
  type Target = T;

  fn deref(&self) -> &T {
    return unsafe { &*(&self.ptr as *const *mut _ as *const T) };
  }
}

fn get_method_ptr<T>(offset: usize) -> Option<MethodPtr<T>> {
  unsafe {
    return Some(MethodPtr {
      ptr: *(offset as *mut usize) as *mut c_void,
      pd: PhantomData
    });
  }
}

#[repr(C)]
pub struct MethodInfo {
  pub invoker_method: *const c_void,
  pub method_pointer: *const c_void
}

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

pub struct Il2Cpp {
  pub unity_player: *mut c_void,
  pub game_assembly: *mut c_void,
  pub functions: Il2CppFunctions
}

impl Il2Cpp {
  pub fn new(path: PathBuf) -> Self {
    let unity_player = Self::load_library(path.join("UnityPlayer.dll"));
    let game_assembly = Self::load_library(path.join("GameAssembly.dll"));
    let functions = Il2CppFunctions::new(unity_player as usize);

    return Il2Cpp {
      unity_player: unity_player.cast(),
      game_assembly: game_assembly.cast(),
      functions: functions
    };
  }

  fn load_library(path: PathBuf) -> *mut c_void {
    let native = CString::new(path.to_str().unwrap()).unwrap();
    let library = unsafe { LoadLibraryA(native.as_ptr()) };
    return library as *mut c_void;
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