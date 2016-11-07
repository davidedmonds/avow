extern crate colored;
extern crate tabwriter;

use std::io::Write;
use tabwriter::TabWriter;

pub mod vec;

pub fn prettify(content: &str) -> String {
  let mut tw = TabWriter::new(Vec::new());
  tw.write(content.as_bytes()).unwrap();
  tw.flush().unwrap();
  String::from_utf8(tw.unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
