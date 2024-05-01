use ctor::ctor;
use winapi::um::consoleapi;
use std::{thread, time::Duration};
use crate::{dumper::HonkaiDumper, il2cpp};

#[ctor]
fn entry() {
  thread::spawn(|| {
    thread::sleep(Duration::from_secs(10));

    unsafe { consoleapi::AllocConsole() };
    println!("honkai-dumper");

    println!("initializing the il2cpp api");
    let api = il2cpp::get_il2cpp_api();

    println!("dumping the methods");
    let dumper = HonkaiDumper::new(&api);
    dumper.dump();
  });
}