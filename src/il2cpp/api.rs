use std::{error::Error, ffi::{c_void, CStr}, path::PathBuf, str::Utf8Error};
use thiserror::Error;
use super::{functions::Il2CppFunctions, module::{Module, ModuleError}, types::MethodInfo};

#[derive(Debug, Error)]
pub enum Il2CppError {
  #[error(transparent)]
  Module(#[from] ModuleError),
  #[error(transparent)]
  Utf8(#[from] Utf8Error),

  #[error("file not found {0}")]
  FileNotFound(&'static str),
  #[error("function not found {0}")]
  FunctionNotFound(&'static str),
  #[error("root path not found")]
  RootNotFound,
  #[error("function returned null {0}")]
  ReturnedNull(&'static str)
}

pub struct Il2CppApi {
  pub game_assembly: Module,
  pub unity_player: Module,
  pub functions: Il2CppFunctions
}

impl Il2CppApi {
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

    Ok(Il2CppApi {
      game_assembly,
      unity_player,
      functions
    })
  }

  pub fn domain_get(&self) -> Result<*const c_void, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_domain_get
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_domain_get"))?;

    let domain = function();

    if domain.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_domain_get"));
    }

    Ok(domain)
  }

  pub fn domain_get_assemblies(&self, domain: *const c_void, size: *const usize) -> Result<*const *const c_void, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_domain_get_assemblies
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_domain_get_assemblies"))?;

    let assemblies = function(domain, size);

    if assemblies.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_domain_get_assemblies"));
    }

    Ok(assemblies)
  }

  pub fn assembly_get_image(&self, assembly: *const c_void) -> Result<*const c_void, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_assembly_get_image
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_assembly_get_image"))?;
      
    let image = function(assembly);

    if image.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_assembly_get_image"));
    }

    Ok(image)
  }

  pub fn image_get_class_count(&self, image: *const c_void) -> Result<usize, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_image_get_class_count
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_image_get_class_count"))?;

    Ok(function(image))
  }

  pub fn image_get_class(&self, image: *const c_void, index: usize) -> Result<*const c_void, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_image_get_class
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_image_get_class"))?;

    let class = function(image, index);

    if class.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_image_get_class"));
    }

    Ok(class)
  }

  pub fn class_get_methods(&self, class: *const c_void, iter: *const *const c_void) -> Result<Option<*const MethodInfo>, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_class_get_methods
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_class_get_methods"))?;

    let result = function(class, iter);

    Ok(if result.is_null() { None } else { Some(result) })
  }

  pub fn class_get_name(&self, class: *const c_void) -> Result<String, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_class_get_name
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_class_get_name"))?;

    let name_c = function(class);

    if name_c.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_class_get_name"));
    }

    let name = unsafe { CStr::from_ptr(name_c) };

    Ok(name.to_str()?.to_string())
  }

  pub fn class_get_namespace(&self, class: *const c_void) -> Result<String, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_class_get_namespace
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_class_get_namespace"))?;

    let name_c = function(class);

    if name_c.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_class_get_namespace"));
    }

    let name = unsafe { CStr::from_ptr(name_c) };

    Ok(name.to_str()?.to_string())
  }

  pub fn method_get_name(&self, method: *const MethodInfo) -> Result<String, Il2CppError> {
    let function = &self
      .functions
      .il2cpp_method_get_name
      .as_ref()
      .ok_or(Il2CppError::FunctionNotFound("il2cpp_method_get_name"))?;

    let name_c = function(method);

    if name_c.is_null() {
      return Err(Il2CppError::ReturnedNull("il2cpp_method_get_name"));
    }

    let name = unsafe { CStr::from_ptr(name_c) };

    Ok(name.to_str()?.to_string())
  }
}

static mut API: Option<Il2CppApi> = None;

pub fn get_il2cpp_api() -> Result<&'static Il2CppApi, Box<dyn Error>> {
  unsafe {
    if API.is_none() {
      let exe_path = std::env::current_exe()?;
      let root_path = exe_path.parent().ok_or(Il2CppError::RootNotFound)?.to_path_buf();
      API = Some(Il2CppApi::new(root_path)?);
    }
  
    Ok(API.as_ref().ok_or("Failed to get the il2cpp api")?)
  }
}