use std::{error::Error, ffi::{c_char, c_void, CStr}, path::PathBuf};
use thiserror::Error;
use super::{functions::Il2CppFunctions, module::{Module, ModuleError}, types::MethodInfo};

#[derive(Debug, Error)]
pub enum Il2CppError {
  #[error(transparent)]
  Module(#[from] ModuleError),

  #[error("failed to find {0}")]
  FileNotFound(&'static str),
  #[error("function not found {0}")]
  FunctionNotFound(&'static str)
}

pub struct Il2Cpp {
  pub game_assembly: Module,
  pub unity_player: Module,
  pub functions: Il2CppFunctions
}

impl Il2Cpp {
  pub fn new(path: PathBuf) -> Result<Self, Il2CppError> {
    let game_assembly_path = path.join("GameAssembly.dll");
    let unity_player_path = path.join("UnityPlayer.dll");

    if !game_assembly_path.exists() {
      return Err(Il2CppError::FileNotFound("GameAssembly.dll"));
    }

    if !unity_player_path.exists() {
      return Err(Il2CppError::FileNotFound("UnityPlayer.dll"));
    }

    let game_assembly = Module::load(game_assembly_path)?;
    let unity_player = Module::load(unity_player_path)?;
    let functions = Il2CppFunctions::new(unity_player.handle as usize);

    Ok(Il2Cpp {
      game_assembly,
      unity_player,
      functions
    })
  }

  fn to_string(native: *const c_char) -> String {
    let str = unsafe { CStr::from_ptr(native) };
    str.to_str().unwrap().to_string()
  }

  pub fn domain_get(&self) -> Result<*const c_void, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_domain_get
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_domain_get"))?;

    Ok(function())
  }

  pub fn domain_get_assemblies(&self, domain: *const c_void, size: *const usize) -> Result<*const *const c_void, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_domain_get_assemblies
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_domain_get_assemblies"))?;

    Ok(function(domain, size))
  }

  pub fn assembly_get_image(&self, assembly: *const c_void) -> Result<*const c_void, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_assembly_get_image
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_assembly_get_image"))?;

    Ok(function(assembly))
  }

  pub fn image_get_class_count(&self, image: *const c_void) -> Result<usize, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_image_get_class_count
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_image_get_class_count"))?;

    Ok(function(image))
  }

  pub fn image_get_class(&self, image: *const c_void, index: usize) -> Result<*const c_void, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_image_get_class
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_image_get_class"))?;

    Ok(function(image, index))
  }

  pub fn class_get_methods(&self, class: *const c_void, iter: *const *const c_void) -> Result<Option<*const MethodInfo>, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_class_get_methods
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_class_get_methods"))?;

    let result = function(class, iter);

    Ok(if !result.is_null() { 
      Some(result)
    } else {
      None
    })
  }

  pub fn class_get_name(&self, class: *const c_void) -> Result<String, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_class_get_name
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_class_get_name"))?;

    Ok(Self::to_string(function(class)))
  }

  pub fn class_get_namespace(&self, class: *const c_void) -> Result<String, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_class_get_namespace
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_class_get_namespace"))?;

    Ok(Self::to_string(function(class)))
  }

  pub fn method_get_name(&self, method: *const MethodInfo) -> Result<String, Il2CppError> {
    let function = &self
      .functions
      .clone()
      .il2cpp_method_get_name
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_method_get_name"))?;

    Ok(Self::to_string(function(method)))
  }
}

static mut API: Option<Il2Cpp> = None;

pub fn get_il2cpp_api() -> Result<&'static Il2Cpp, Box<dyn Error>> {
  unsafe {
    if API.is_none() {
      API = Some(Il2Cpp::new(std::env::current_exe()?.parent().unwrap().to_path_buf())?);
    }
  
    Ok(API.as_ref().ok_or("Failed to get the il2cpp api")?)
  }
}