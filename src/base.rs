use ctor::ctor;
use winapi::um::consoleapi;
use std::{error::Error, thread, time::Duration};

use crate::outputs::{csdumper, methoddumper};

fn init() -> Result<(), Box<dyn Error>> {
  thread::sleep(Duration::from_secs(10));

  unsafe { consoleapi::AllocConsole() };
  println!("honkai-dumper");

  println!("dumping");

  // Use the following to only dump the method offsets
  methoddumper::dump()?;

  // Use the following to dump all classes, fields and methods
  // csdumper::dump()?;

  println!("done");

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