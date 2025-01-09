use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    message: String,
}

fn to_and_from_json() {
    let json = json!({"message": "Hello, World!"});
    let my_struct: MyStruct = serde_json::from_str(&json).unwrap();
}

pub fn master(show: bool) {
    if show {
        println!("Serde Tutorial");
    }
}