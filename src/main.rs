use std::io;
use std::io::File;
use std::os;
use std::str;

fn main() {
  println!("meh");
  let filename=&os::args()[1];
  let contents = match File::open(&Path::new(filename)).read_to_end() {
  	Ok(s) => str::from_utf8(s.as_slice()).expect("this shouldn't happen").to_string(),
  	Err(e) => "".to_string()
  };
  println!("ugh {}", contents.to_string());
}

