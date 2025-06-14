mod parser;

use std::fs::File;
use parser::JsonParser;

fn main() {
    let file = File::open("./data.json").expect("ファイルを開けません。");
    let value = JsonParser::parse(file).expect("無理でした。");
    
    /*
    let string = r#"
    {
        "i64": 8623,
        "f64": 86.23,
        "string": "success",
        "true": true,
        "false": false,
        "array": [0, 1, 2, 3, 4],
        "null": null
    }
    "#;

    let value = JsonParser::parse_from_bytes(string.as_bytes());
    */

    println!("{:?}", value);
}
