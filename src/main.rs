use serde::{Serialize,Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize,Debug)]
struct Point{
    x:i32,
    y:i32
}
fn from_json()->Result<()>{
    let data = r#"{"x":1,"y":2}"#;
    let p:Point =serde_json::from_str(data)?;
    println!("deserialized={:?}",p);
    Ok(())
}
fn create_json(){
    let json =serde_json::json!({
        "code":200,
        "success":true,
        "payload":{
            "features":["serde","json"],
            "ids":[1,2,3]
        }
    });
    println!("json={}",json);
}

fn modify_json(){
    let mut json = serde_json::json!({"ferris":"hello"});
    json["ferris"] = serde_json::json!("world");
    assert_eq!("world",json["ferris"])
}
fn main() {
    from_json().unwrap_or_else(|e| {
        eprintln!("Error deserializing JSON: {}", e);
        std::process::exit(1);
    });
    create_json();
    modify_json();
}
