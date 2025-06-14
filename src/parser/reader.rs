use std::collections::VecDeque;
use std::io::{BufReader, Cursor, Read, Seek};
use std::str::from_utf8;

pub struct JsonReader<T> where T: Read + Seek {
    reader: BufReader<T>,
    character_buffer: VecDeque<char>,
}

impl<T> JsonReader<T> where T: Read + Seek {
    pub fn new(reader: BufReader<T>) -> Self {
        JsonReader {
            reader,
            character_buffer: VecDeque::with_capacity(4),
        }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> JsonReader<Cursor<&[u8]>> {
        JsonReader {
            reader: BufReader::new(Cursor::new(bytes)),
            character_buffer: VecDeque::with_capacity(4),
        }
    }
}

impl<T> Iterator for JsonReader<T> where T: Read + Seek {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.character_buffer.is_empty() {
            let mut utf8_buffer = [0u8; 4];
            let _ = self.reader.read(&mut utf8_buffer);

            match from_utf8(&utf8_buffer) {
                Ok(string) => {
                    self.character_buffer = string.chars().collect();
                }

                Err(error) => {
                    let valid_bytes = error.valid_up_to();
                    let string = from_utf8(&utf8_buffer[..valid_bytes]).unwrap();

                    let remaining_bytes = 4 - valid_bytes;
                    let _ = self.reader.seek_relative(-(remaining_bytes as i64));

                    self.character_buffer = string.chars().collect();
                }
            }
        }

        match self.character_buffer.pop_front() {
            Some(c) if c != '\0' => Some(c),
            _ => None,
        }
    }
}
