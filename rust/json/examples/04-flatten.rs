// Flatten an object to a property as part of a struct
//
// Structs require fields to access data.
//
// If a JSON object is like a HashMap, we can use flatten to assign it to a
// specific field.
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Equipment {
    #[serde(flatten)]
    equipped: HashMap<String, String>,
}

fn main() -> Result<()> {
    let data = r#"
        {
            "weapon": "Erdrick's Sword",
            "armor": "Erdrick's Armor",
            "shield": "Silver Shield"
        }
    "#;
    let equipment: Equipment = serde_json::from_str(data)?;
    println!("{:#?}", equipment);

    println!("{}", serde_json::to_string_pretty(&equipment).unwrap());

    // direct to HashMap
    let equipment: HashMap<String, String> = serde_json::from_str(data)?;
    println!("{:#?}", equipment);

    Ok(())
}
