// Optional values
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Vehicle {
    name: String,
    wheels: u8,
    alias: Option<String>,
}

fn main() -> Result<()> {
    let data = r#"
        {
            "name": "Vehicle",
            "wheels": 4
        }
    "#;
    let c: Vehicle = serde_json::from_str(data)?;
    println!("{:#?}", c);

    let data = r#"
        {
            "name": "Truck",
            "wheels": 18,
            "alias": "Mac"
        }
    "#;
    let c: Vehicle = serde_json::from_str(data)?;
    println!("{:#?}", c);

    Ok(())
}
