// Parse arbitrary JSON
//
// https://github.com/serde-rs/json#operating-on-untyped-json-values
// https://docs.serde.rs/serde_json/value/enum.Value.html
//
// enum Value {
//    Null,
//    Bool(bool),
//    Number(Number),
//    String(String),
//    Array(Vec<Value>),
//    Object(Map<String, Value>),
// }
use serde_json::Value;

fn main() {
    let json = r#"
        {
          "name": "John Doe",
          "age": 43,
          "phones": [
            "+44 1234567",
            "+44 2345678"
          ]
        }
    "#;
    let cmd: Value = serde_json::from_str(json).unwrap();

    println!("serde_json::Value");
    println!("{:#?}", cmd);
    println!("");
    println!("serde_json::to_string");
    println!("{}", serde_json::to_string(&cmd).unwrap());
}
