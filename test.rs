extern crate std;
extern crate native;
extern crate debug;

use native::io::file::open;
use std::rt::rtio::{Open, Read};
use std::os::{MemoryMap, MapReadable, MapWritable, MapFd};

fn main() {
  let path = "test.bin";
  let file =
    match open(&path.to_c_str(), Open, Read) {
      Err(_) => { fail!("Something is terribly wrong!"); },
      Ok(f)  => { f },
    };
  let fmap = 
    match MemoryMap::new(32u, [MapReadable, MapWritable, MapFd(file.fd())]) {
      Err(_) => { fail!("Something is terribly wrong!"); },
      Ok(f)  => { f },
    };
  let test: &'static [u8] = b"Hello, World!\n";
  //for i in range(0, fmap.len()) {
  for i in range(0u, 14) {
    unsafe {
      //println!("data[{}]=`{}`", i, *(fmap.data().offset(i as int)) as char);
      assert_eq!(*(fmap.data().offset(i as int)), test[i] as u8);
    }
  }
}
