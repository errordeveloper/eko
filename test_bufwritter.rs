extern crate serialize;
extern crate debug;

use std::io::BufWriter;
use serialize::json;

#[deriving(Decodable, Encodable)]
pub struct Record {
  version: String,
  schema: String,
  data: Data,
}

#[deriving(Decodable, Encodable)]
pub struct Data {
  test: bool,
  uuid: String,
  time: f64,
}

fn main() {
  let mut buf = [0, ..4096];

  let data = json::encode(&Record {
    version: "0.1".to_string(),
    schema: "test".to_string(),
    data: Data {
      test: true,
      uuid: "7D315030-F59C-430F-8E1A-0A8A59EAC8EB".to_string(),
      time: 1405506742.001483,
    },
  });
  {
    match BufWriter::new(buf).write(data.as_bytes()) {
      Ok(x) => assert_eq!(x, ()),
      Err(e) => fail!(e),
    }
  }
  println!("{:?}", buf.slice(0, data.len()));
}
