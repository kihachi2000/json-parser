mod parser;

use std::io::{Cursor};
use parser::reader::JsonReader;

fn main() {
    let string = "abc";
    let reader = JsonReader::<Cursor<&'static[u8]>>::from_bytes(string.as_bytes());

    for c in reader.into_iter() {
        println!("{}", c);
    }
}
