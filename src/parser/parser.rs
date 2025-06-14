use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::iter::Peekable;
use std::slice::Iter;
use crate::parser::token::{JsonTokenizer, Token};
use crate::parser::value::Value;

pub struct JsonParser;

impl JsonParser {
    pub fn parse_from_bytes<'a>(input: &'a [u8]) -> Result<Value, ()> {
        let mut json_tokenizer = JsonTokenizer::<Cursor<&[u8]>>::from_bytes(input);
        let tokens = json_tokenizer.tokenize_json()?;

        Ok(Self::tokens_to_value(tokens))
    }

    pub fn parse(reader: File) -> Result<Value, ()> {
        let mut json_tokenizer = JsonTokenizer::<File>::new(reader);
        let tokens = json_tokenizer.tokenize_json()?;

        Ok(Self::tokens_to_value(tokens))
    }

    fn tokens_to_value(tokens: &[Token]) -> Value {
        let mut iterator = tokens.iter().peekable();
        Self::get_value(&mut iterator)
    }

    fn get_value(iterator: &mut Peekable<Iter<Token>>) -> Value {
        if let Some(token) = iterator.next() {
            match token {
                Token::CurlyOpen => {
                    Value::Object(Self::process_object(iterator))
                },

                Token::ArrayOpen => {
                    Value::Array(Self::process_array(iterator))
                },

                Token::Quotes => {
                    Value::String(Self::process_string(iterator))
                },

                Token::Number(number) => {
                    Value::Number(*number)
                },

                Token::Boolean(boolean) => {
                    Value::Boolean(*boolean)
                },

                Token::Null => {
                    Value::Null
                },

                invalid => {
                    panic!("Unexpected token found: {:?}", invalid);
                },
            }
        } else {
            panic!("Token not found");
        }
    }

    fn process_string(iterator: &mut Peekable<Iter<Token>>) -> String {
        let string = iterator.next().unwrap();
        let quote = iterator.next().unwrap();

        if let (Token::String(string), Token::Quotes) = (string, quote) {
            string.clone()
        } else {
            panic!("Something went wrong while process string");
        }
    }

    fn process_object(iterator: &mut Peekable<Iter<Token>>) -> HashMap<String, Value> {
        let mut obj = HashMap::<String, Value>::new();

        loop {
            let _ = iterator.next().unwrap();
            let key = iterator.next().unwrap();
            let _ = iterator.next().unwrap();
            let colon = iterator.next().unwrap();
            let value = Self::get_value(iterator);
            let separate_mark = iterator.next().unwrap();

            if let Token::Colon = colon {
            } else {
                panic!("Colon not found");
            }

            if let Token::String(key) = key {
                obj.insert(key.clone(), value);
            }

            match separate_mark {
                Token::Comma => {
                    continue;
                },

                Token::CurlyClose => {
                    break;
                },

                _ => {
                    panic!("Unexpected token found while process an object");
                }
            }
        }

        obj
    }

    fn process_array(iterator: &mut Peekable<Iter<Token>>) -> Vec<Value> {
        let mut arr = Vec::new();

        loop {
            let value = Self::get_value(iterator);
            let separate_mark = iterator.next().unwrap();

            arr.push(value);

            match separate_mark {
                Token::Comma => {
                    continue;
                },

                Token::ArrayClose => {
                    break;
                },

                _ => {
                    panic!("Unexpected token found while process an object");
                }
            }
        }

        arr
    }
}
