use std::{ffi::c_void, fs::File, ptr::null};
use serde_json::{json, Value};
use crate::il2cpp::Il2Cpp;

#[derive(Clone, Copy)]
pub struct HonkaiDumper<'a> {
  pub il2cpp_api: &'a Il2Cpp
}

impl<'a> HonkaiDumper<'a> {
  pub fn new(api: &'a Il2Cpp) -> Self {
    return HonkaiDumper {
      il2cpp_api: api
    };
  }

  fn replace_special_chars(string: &mut String) {
    for i in vec!['<', '>', '`'] {
      *string = string.replace(i, "_");
    }
  }

  fn check_repeats(original: &str, value: &Value) -> String {
    let mut counted = original.to_string();
    let mut occurences = 0;

    while !value[counted.as_str()].is_null() {
      counted = format!("{}_{}", original, occurences);
      occurences += 1;
    }

    return counted;
  }

  pub fn dump(&self) {
    let mut output = json!({});
    let mut count = 0;

    let domain = self.il2cpp_api.domain_get();

    let assembly_count: usize = 0;
    let assemblies = self.il2cpp_api.domain_get_assemblies(domain, &assembly_count);

    for i in 0..assembly_count {
      let assembly = unsafe { *assemblies.offset(i as isize) };

      if assembly.is_null() {
        continue;
      }

      let image = self.il2cpp_api.assembly_get_image(assembly);
      let class_count = self.il2cpp_api.image_get_class_count(image);

      for j in 0..class_count {
        let class = self.il2cpp_api.image_get_class(image, j);

        if class.is_null() {
          continue;
        }

        let mut class_name = self.il2cpp_api.class_get_name(class);
        Self::replace_special_chars(&mut class_name);
        
        let mut class_namespace = self.il2cpp_api.class_get_namespace(class);

        if !class_namespace.is_empty() {
          class_namespace.push('.');
        }

        let method_iter: *const c_void = null();
  
        while let Some(method_info) = self.il2cpp_api.class_get_methods(class, &method_iter) {
          let mut method_name = self.il2cpp_api.method_get_name(method_info);
          Self::replace_special_chars(&mut method_name);

          let description = format!("{}{}::{}", class_namespace, class_name, method_name);
          let counted = Self::check_repeats(description.as_str(), &output);
          let pointer = unsafe { (*method_info).method_pointer as usize - self.il2cpp_api.game_assembly as usize };
          output[counted] = json!(format!("0x{:x}", pointer));

          count += 1;
        }
      }
    }

    let mut file = File::create("methods.json").unwrap();
    serde_json::to_writer_pretty(&mut file, &output).unwrap();

    println!("{} methods found and saved to methods.json", count);
    println!("done");
  }
}