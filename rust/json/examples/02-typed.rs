// Serialize/Deserialize JSON data into Rust structs
//
// Uses derive macros to setup required traits for easy use in rust.
// https://serde.rs/derive.html
//
// Enable using the "derive" feature in Cargo.toml
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
//
// Extended example from:
// https://github.com/serde-rs/json#operating-on-untyped-json-values
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    tall: bool,
    // array
    phones: Vec<String>,
    // object
    addresses: HashMap<String, Address>,
}

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn main() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "tall": true,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "addresses": {
                "home": {
                    "street": "123 Sesame St",
                    "city": "Manhattan"
                },
                "work": {
                    "street": "34-12 36th St",
                    "city": "Queens"
                }
            }
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);
    println!(
        "Lives at {} in {}",
        p.addresses["home"].street, p.addresses["home"].city
    );
    println!(
        "Works at {} in {}",
        p.addresses["work"].street, p.addresses["work"].city
    );
    if p.tall {
        println!("{} is tall", p.name);
    }

    Ok(())
}
