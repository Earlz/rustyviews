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
  pub fn generate(&self, output: &mut Writer) -> String {
    for c in self.template.chars() {
      output.write_char(c);
    }
    "".to_string()
  }
}