use std::str;
use std::io;

pub struct ViewEngine {
  pub template: String
}
impl ViewEngine {
	pub fn new(template: String) -> ViewEngine {
		let e=ViewEngine{ template: template};
		e
	}
	pub fn generate(output: &Writer) -> String {

		"".to_string()
	}
}