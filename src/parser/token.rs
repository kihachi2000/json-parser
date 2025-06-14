use std::io::{BufReader, Cursor, Read, Seek};
use std::fs::File;
use std::iter::Peekable;
use crate::parser::reader::JsonReader;
use crate::parser::value::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(Number),
    Boolean(bool),
    Null,

    CurlyOpen,
    CurlyClose,
    ArrayOpen,
    ArrayClose,
    Quotes,
    Colon,
    Comma,
}

pub struct JsonTokenizer<T> where T: Read + Seek {
    tokens: Vec<Token>,
    iterator: Peekable<JsonReader<T>>,
}

impl<T> JsonTokenizer<T> where T: Read + Seek {
    pub fn new(reader: File) -> JsonTokenizer<File> {
        let json_reader = JsonReader::<File>::new(BufReader::new(reader));

        JsonTokenizer {
            tokens: Vec::<Token>::new(),
            iterator: json_reader.peekable(),
        }
    }

    pub fn from_bytes<'a>(input: &'a [u8]) -> JsonTokenizer<Cursor<&'a [u8]>> {
        let json_reader = JsonReader::<Cursor<&'a [u8]>>::from_bytes(input);

        JsonTokenizer {
            tokens: Vec::<Token>::with_capacity(input.len()),
            iterator: json_reader.peekable(),
        }
    }

    // 引数に&mut selfが入っているためライフタイム注釈は省略可能
    // 自動的にselfのライフタイムが割り当てられる
    pub fn tokenize_json(&mut self) -> Result<&[Token], ()> {
        while let Some(c) = self.iterator.peek() {
            match *c {
                '"' => {
                    self.tokens.push(Token::Quotes);
                    let _ = self.iterator.next();

                    let string = self.tokenize_string();

                    self.tokens.push(Token::String(string));
                    self.tokens.push(Token::Quotes);
                },

                '-' | '0'..='9' => {
                    let number = self.tokenize_number();
                    self.tokens.push(Token::Number(number));
                },

                't' => {
                    let boolean = self.tokenize_true();
                    self.tokens.push(Token::Boolean(boolean));
                },

                'f' => {
                    let boolean = self.tokenize_false();
                    self.tokens.push(Token::Boolean(boolean));
                },

                'n' => {
                    self.tokenize_null();
                    self.tokens.push(Token::Null);
                },

                '{' => {
                    self.tokens.push(Token::CurlyOpen);
                    let _ = self.iterator.next();
                },

                '}' => {
                    self.tokens.push(Token::CurlyClose);
                    let _ = self.iterator.next();
                },

                '[' => {
                    self.tokens.push(Token::ArrayOpen);
                    let _ = self.iterator.next();
                },

                ']' => {
                    self.tokens.push(Token::ArrayClose);
                    let _ = self.iterator.next();
                },

                ',' => {
                    self.tokens.push(Token::Comma);
                    let _ = self.iterator.next();
                }

                ':' => {
                    self.tokens.push(Token::Colon);
                    let _ = self.iterator.next();
                }

                ' ' | '\t' | '\r' | '\n' => {
                    let _ = self.iterator.next();
                    continue;
                },

                '\0' => {
                    break;
                }

                invalid => {
                    panic!("Unexpected character: {}", invalid);
                },
            }
        }

        Ok(&self.tokens)
    }

    fn tokenize_string(&mut self) -> String {
        let mut chars = Vec::<char>::new();

        while let Some(c) = self.iterator.peek() {
            match *c {
                '"' => {
                    let _ = self.iterator.next();
                    break;
                },

                c => {
                    let _ = self.iterator.next();
                    chars.push(c);
                },
            }
        }

        String::from_iter(chars)
    }

    fn tokenize_number(&mut self) -> Number {
        let mut chars = Vec::<char>::new();
        let mut is_decimal = true;

        while let Some(c) = self.iterator.peek() {
            match *c {
                '-' | '0'..='9' => {
                    chars.push(*c);
                    let _ = self.iterator.next();
                },

                '.' => {
                    chars.push(*c);
                    let _ = self.iterator.next();
                    is_decimal = false;
                }
                
                _ => {
                    break;
                }
            }
        }

        if is_decimal == true {
            Number::I64(String::from_iter(chars).parse::<i64>().unwrap())
        } else {
            Number::F64(String::from_iter(chars).parse::<f64>().unwrap())
        }
    }

    fn tokenize_true(&mut self) -> bool {
        assert_eq!(Some('t'), self.iterator.next());
        assert_eq!(Some('r'), self.iterator.next());
        assert_eq!(Some('u'), self.iterator.next());
        assert_eq!(Some('e'), self.iterator.next());
        true
    }

    fn tokenize_false(&mut self) -> bool {
        assert_eq!(Some('f'), self.iterator.next());
        assert_eq!(Some('a'), self.iterator.next());
        assert_eq!(Some('l'), self.iterator.next());
        assert_eq!(Some('s'), self.iterator.next());
        assert_eq!(Some('e'), self.iterator.next());
        false
    }

    fn tokenize_null(&mut self) {
        assert_eq!(Some('n'), self.iterator.next());
        assert_eq!(Some('u'), self.iterator.next());
        assert_eq!(Some('l'), self.iterator.next());
        assert_eq!(Some('l'), self.iterator.next());
    }
}
