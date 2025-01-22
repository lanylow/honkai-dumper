use ctor::ctor;
use winapi::um::consoleapi;
use std::{error::Error, thread, time::Duration};
use crate::dumper;

fn init() -> Result<(), Box<dyn Error>> {
  thread::sleep(Duration::from_secs(10));

  unsafe { consoleapi::AllocConsole() };
  println!("honkai-dumper");

  println!("dumping methods");
  dumper::dump()?;

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