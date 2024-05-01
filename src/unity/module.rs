use std::{ffi::{c_void, CString}, marker::PhantomData, mem::size_of, ops::Deref, path::PathBuf};
use winapi::um::{libloaderapi::LoadLibraryA, processthreadsapi::GetCurrentProcess, psapi::{GetModuleInformation, MODULEINFO}};

pub struct Module {
  pub handle: *mut c_void,
  pub size: usize
}

impl Module {
  pub fn load(path: PathBuf) -> Self {
    unsafe {
      let native = CString::new(path.to_str().unwrap()).unwrap();
      let handle = LoadLibraryA(native.as_ptr());
  
      let mut module_info: MODULEINFO = std::mem::zeroed();
      GetModuleInformation(GetCurrentProcess(), handle.cast(), &mut module_info, size_of::<MODULEINFO>() as u32);
      let size = module_info.SizeOfImage as usize;

      return Module {
        handle: handle.cast(),
        size: size
      }
    };
  }
}

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

impl<T> Clone for MethodPtr<T> {
  fn clone(&self) -> Self {
    return MethodPtr { ..*self };
  }
}

pub fn get_method_ptr<T>(offset: usize) -> Option<MethodPtr<T>> {
  unsafe {
    return Some(MethodPtr {
      ptr: *(offset as *mut usize) as *mut c_void,
      pd: PhantomData
    });
  }
}