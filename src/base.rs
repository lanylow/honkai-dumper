use ctor::ctor;
use winapi::um::consoleapi;
use std::{error::Error, thread, time::Duration};
use crate::{dumper::HonkaiDumper, unity::api};

fn init() -> Result<(), Box<dyn Error>> {
  thread::sleep(Duration::from_secs(10));

  unsafe { consoleapi::AllocConsole() };
  println!("honkai-dumper");

  println!("initializing the il2cpp api");
  let api = api::get_il2cpp_api()?;

  println!("dumping the methods");
  let dumper = HonkaiDumper::new(&api);
  dumper.dump()?;

  Ok(())
}

#[ctor]
fn entry() {
  thread::spawn(|| {
    init().unwrap_or_else(|e| {
      panic!("honkai-dumper encountered an error: {}", e)
    });
  });
}