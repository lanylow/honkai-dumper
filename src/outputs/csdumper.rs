use std::{error::Error, ffi::c_void, fs::File, io::Write, ptr::null};

use crate::il2cpp::{api, constants::*};

fn write_images() -> Result<String, Box<dyn Error>> {
  let il2cpp = api::get_il2cpp_api()?;
  let mut output = String::new();

  let domain = il2cpp.domain_get()?;

  let assembly_size: usize = 0;
  let assemblies = il2cpp.domain_get_assemblies(domain, &assembly_size)?;

  for i in 0..assembly_size {
    let assembly = unsafe { *assemblies.offset(i as isize) };

    if assembly.is_null() {
      continue;
    }

    let image = il2cpp.assembly_get_image(assembly)?;
    let name = il2cpp.image_get_name(image)?;

    let fmt = format!("// Image {}: {}\n", i, name);
    output.push_str(fmt.as_str());
  }

  Ok(output)
}

fn write_fields(class: *const c_void) -> Result<String, Box<dyn Error>> {
  let il2cpp = api::get_il2cpp_api()?;
  let mut output = String::new();

  output.push_str("\n\t// Fields\n");

  let field_iter: *const c_void = null();

  while let Some(field) = il2cpp.class_get_fields(class, &field_iter)? {
    output.push_str("\t");

    let flags = il2cpp.field_get_flags(field)?;
    let access = flags & FIELD_ATTRIBUTE_FIELD_ACCESS_MASK;

    let access_str = match access {
      FIELD_ATTRIBUTE_PRIVATE => "private ",
      FIELD_ATTRIBUTE_PUBLIC => "public ",
      FIELD_ATTRIBUTE_FAMILY => "protected ",
      FIELD_ATTRIBUTE_ASSEMBLY | FIELD_ATTRIBUTE_FAM_AND_ASSEM => "internal ",
      FIELD_ATTRIBUTE_FAM_OR_ASSEM => "protected internal ",
      _ => ""
    };

    output.push_str(access_str);

    if flags & FIELD_ATTRIBUTE_LITERAL != 0 {
      output.push_str("const ");
    }
    else {
      if flags & FIELD_ATTRIBUTE_STATIC != 0 {
        output.push_str("static ");
      }

      if flags & FIELD_ATTRIBUTE_INIT_ONLY != 0 {
        output.push_str("readonly ");
      }
    }

    let field_type = il2cpp.field_get_type(field)?;
    let field_offset = il2cpp.field_get_offset(field)?;

    let type_name = il2cpp.type_get_name(field_type)?;
    let field_name = il2cpp.field_get_name(field)?;

    let fmt = format!("{} {}; // 0x{:x}\n", type_name, field_name, field_offset);
    output.push_str(fmt.as_str());
  }

  Ok(output)
}

fn write_methods(class: *const c_void) -> Result<String, Box<dyn Error>> {
  let il2cpp = api::get_il2cpp_api()?;
  let mut output = String::new();

  output.push_str("\n\t// Methods\n");

  let method_iter: *const c_void = null();

  while let Some(method) = il2cpp.class_get_methods(class, &method_iter)? {
    output.push_str("\n");

    let pointer = unsafe { (*method).method_pointer as usize };

    if pointer != 0 {
      let offset = pointer - il2cpp.game_assembly.handle as usize;
      let fmt = format!("\t// RVA: 0x{:x} VA: 0x{:x}\n\t", offset, offset + 0x180000000);
      output.push_str(fmt.as_str());
    }
    else {
      output.push_str("\t// RVA: 0x0 VA: 0x0\n\t");
    }

    let flags = unsafe { (*method).flags } as i32;
    let access = flags & METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK;

    let access_str = match access {
      METHOD_ATTRIBUTE_PRIVATE => "private ",
      METHOD_ATTRIBUTE_PUBLIC => "public ",
      METHOD_ATTRIBUTE_FAMILY => "protected ",
      METHOD_ATTRIBUTE_ASSEM | METHOD_ATTRIBUTE_FAM_AND_ASSEM => "internal ",
      METHOD_ATTRIBUTE_FAM_OR_ASSEM => "protected internal ",
      _ => ""
    };

    output.push_str(access_str);

    if flags & METHOD_ATTRIBUTE_STATIC != 0 {
      output.push_str("static ");
    }

    if flags & METHOD_ATTRIBUTE_ABSTRACT != 0 {
      output.push_str("abstract ");

      if flags & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_REUSE_SLOT {
        output.push_str("override ");
      }
    }
    else if flags & METHOD_ATTRIBUTE_FINAL != 0 && flags & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_REUSE_SLOT {
      output.push_str("sealed override ");
    }
    else if flags & METHOD_ATTRIBUTE_VIRTUAL != 0 {
      if flags & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_NEW_SLOT {
        output.push_str("virtual ");
      }
      else {
        output.push_str("override ");
      }
    }

    if flags & METHOD_ATTRIBUTE_PINVOKE_IMPL != 0 {
      output.push_str("extern ");
    }

    let return_type = il2cpp.method_get_return_type(method)?;

    if il2cpp.type_is_byref(return_type)? {
      output.push_str("ref ");
    }

    let return_name = il2cpp.type_get_name(return_type)?;
    let method_name = il2cpp.method_get_name(method)?;

    let fmt = format!("{} {}(", return_name, method_name);
    output.push_str(fmt.as_str());

    let param_count = il2cpp.method_get_param_count(method)?;

    for i in 0..param_count {
      let param = il2cpp.method_get_param(method, i)?;
      let attrs = il2cpp.type_get_attrs(param)? as i32;

      if il2cpp.type_is_byref(param)? {
        if attrs & PARAM_ATTRIBUTE_OUT != 0 && attrs & PARAM_ATTRIBUTE_IN == 0 {
          output.push_str("out ");
        }
        else if attrs & PARAM_ATTRIBUTE_IN != 0 && attrs & PARAM_ATTRIBUTE_OUT == 0 {
          output.push_str("in ");
        }
        else {
          output.push_str("ref ");
        }
      }
      else {
        if attrs & PARAM_ATTRIBUTE_IN != 0 {
          output.push_str("[In] ");
        }

        if attrs & PARAM_ATTRIBUTE_OUT != 0 {
          output.push_str("[Out] ");
        }
      }

      let type_name = il2cpp.type_get_name(param)?;
      let fmt = format!("{}{}", type_name, if i != param_count - 1 { ", " } else { "" });
      output.push_str(fmt.as_str());
    }

    output.push_str(") { }\n");
  }

  Ok(output)
}

fn write_class(class: *const c_void) -> Result<String, Box<dyn Error>> {
  let il2cpp = api::get_il2cpp_api()?;
  let mut output = String::new();

  let namespace = il2cpp.class_get_namespace(class)?;
  let fmt = format!("\n// Namespace: {}\n", namespace);
  output.push_str(fmt.as_str());

  let flags = il2cpp.class_get_flags(class)?;

  if flags & TYPE_ATTRIBUTE_SERIALIZABLE != 0 {
    output.push_str("[Serializable]\n");
  }

  let visibility = flags & TYPE_ATTRIBUTE_VISIBILITY_MASK;

  let visibility_str = match visibility {
    TYPE_ATTRIBUTE_PUBLIC | TYPE_ATTRIBUTE_NESTED_PUBLIC => "public ",
    TYPE_ATTRIBUTE_NOT_PUBLIC | TYPE_ATTRIBUTE_NESTED_FAM_AND_ASSEM | TYPE_ATTRIBUTE_NESTED_ASSEMBLY => "internal ",
    TYPE_ATTRIBUTE_NESTED_PRIVATE => "private ",
    TYPE_ATTRIBUTE_NESTED_FAMILY => "protected ",
    TYPE_ATTRIBUTE_NESTED_FAM_OR_ASSEM => "protected internal ",
    _ => ""
  };

  output.push_str(visibility_str);

  let is_valuetype = il2cpp.class_is_valuetype(class)?;
  let is_enum = il2cpp.class_is_enum(class)?;

  if flags & TYPE_ATTRIBUTE_ABSTRACT != 0 && flags & TYPE_ATTRIBUTE_SEALED != 0 {
    output.push_str("static ");
  }
  else if !(flags & TYPE_ATTRIBUTE_INTERFACE != 0) && flags & TYPE_ATTRIBUTE_ABSTRACT != 0 {
    output.push_str("abstract ");
  }
  else if !is_valuetype && !is_enum && flags & TYPE_ATTRIBUTE_SEALED != 0 {
    output.push_str("sealed ");
  }

  if flags & TYPE_ATTRIBUTE_INTERFACE != 0 {
    output.push_str("interface ");
  }
  else if is_enum {
    output.push_str("enum ");
  }
  else if is_valuetype {
    output.push_str("struct ");
  }
  else {
    output.push_str("class ");
  }

  let name = il2cpp.class_get_name(class)?;
  output.push_str(name.as_str());

  if let Some(parent) = il2cpp.class_get_parent(class).ok() {
    let name = il2cpp.class_get_name(parent)?;
    let fmt = format!(" : {}", name);
    output.push_str(fmt.as_str());
  }

  output.push_str("\n{");
  output.push_str(write_fields(class)?.as_str());
  output.push_str(write_methods(class)?.as_str());
  output.push_str("}\n");

  Ok(output)
}

fn write_classes() -> Result<String, Box<dyn Error>> {
  let il2cpp = api::get_il2cpp_api()?;
  let mut output = String::new();

  let domain = il2cpp.domain_get()?;

  let assembly_size: usize = 0;
  let assemblies = il2cpp.domain_get_assemblies(domain, &assembly_size)?;

  for i in 0..assembly_size {
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

      output.push_str(write_class(class)?.as_str());
    }
  }

  Ok(output)
}

pub fn dump() -> Result<(), Box<dyn Error>> {
  let mut output = String::new();

  output.push_str(write_images()?.as_str());
  output.push_str(write_classes()?.as_str());

  let mut file = File::create("dump.cs")?;
  file.write_all(output.as_bytes())?;

  Ok(())
}