mod reader;

use std::io::{Cursor};
use reader::json_reader::JsonReader;

fn main() {
    let string = "abc";
    let mut reader = JsonReader::<Cursor<&'static[u8]>>::from_bytes(string.as_bytes());

    for c in reader.into_iter() {
        println!("{}", c);
    }
}
