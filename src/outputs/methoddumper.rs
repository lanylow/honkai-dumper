use std::{collections::HashMap, error::Error, ffi::c_void, fs::File, ptr::null};
use serde_json::json;

use crate::il2cpp::api::{self, Il2CppApi};

fn verify_pointer(il2cpp: &Il2CppApi, pointer: usize) -> bool {
  (pointer > il2cpp.game_assembly.handle as usize) &&
    (pointer < il2cpp.game_assembly.handle as usize + il2cpp.game_assembly.size)
}

pub fn dump() -> Result<(), Box<dyn Error>> {
  let il2cpp = api::get_il2cpp_api()?;
  
  let mut name_map: HashMap<String, usize> = HashMap::new();
  let mut duplicates: HashMap<String, u32> = HashMap::new();

  let domain = il2cpp.domain_get()?;

  let assembly_count: usize = 0;
  let assemblies = il2cpp.domain_get_assemblies(domain, &assembly_count)?;

  for i in 0..assembly_count {
    let assembly = unsafe { *assemblies.offset(i as isize) };

    if assembly.is_null() {
      continue;
    }

    let image = il2cpp.assembly_get_image(assembly)?;
    let class_count = il2cpp.image_get_class_count(image)?;

    for j in 0..class_count {
      let class = il2cpp.image_get_class(image, j)?;

      if class.is_null() {
        continue;
      }

      let class_name = il2cpp.class_get_name(class)?;
      let mut class_namespace = il2cpp.class_get_namespace(class)?;

      if !class_namespace.is_empty() {
        class_namespace.push('.');
      }

      let method_iter: *const c_void = null();

      while let Some(method_info) = il2cpp.class_get_methods(class, &method_iter)? {
        let pointer = unsafe { (*method_info).method_pointer as usize };

        if !verify_pointer(il2cpp, pointer) {
          continue;
        }

        let method_name = il2cpp.method_get_name(method_info)?;
        let description = format!("{}{}::{}", class_namespace, class_name, method_name);

        let unique_description = if name_map.contains_key(&description) {
          let count = duplicates.entry(description.to_string()).or_insert(0);
          *count += 1;
          format!("{}_{}", description, count)
        } else {
          description
        };

        name_map.insert(unique_description, pointer - il2cpp.game_assembly.handle as usize);
      }
    }
  }

  let mut output = json!({});

  for (name, address) in &name_map {
    output[name] = json!(format!("0x{:x}", address));
  }

  let mut file = File::create("methods.json").unwrap();
  serde_json::to_writer_pretty(&mut file, &output).unwrap();

  println!("{} valid methods found and saved to methods.json", name_map.len());

  Ok(())
}