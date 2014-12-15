use std::io;
use std::io::File;
use std::os;
use std::str;
use engine::ViewEngine;
mod engine;
fn main() {
  println!("meh");
  let filename=&os::args()[1];
  let contents = match File::open(&Path::new(filename)).read_to_string() {
  	Ok(s) => s,
  	Err(e) => panic!("Could not open template! {}", e)
  };
  println!("ugh {}", contents);
  let x=ViewEngine::new(contents);
}



