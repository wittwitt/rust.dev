use std::str;
use std::string::String;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn main() {
    {
        let msg = Msg {
            name: "zs".to_string(),
            name2: "zs2".to_string(),
            age: 10,
            age2: 12,
            age3: 13,
            age4: 14,
            age5: 15,
            age6: 16,
        };
        let data = serde_json::to_vec(&msg).unwrap();
        let data_str = String::from(str::from_utf8(&data).unwrap());
        println!("{}", data_str);
    }

    let str = r#"
{
    "nike_name":"zs2",
    "name":"zs",
    "age":10,
    "age2":102,
    "age3":103,
    "age4":104,
    "age5":105,
    "age6":"106"
}
"#;
    let msg2: Msg = serde_json::from_str(str).unwrap();
    println!("{:?}", msg2);
}

#[derive(Deserialize, Serialize, Debug)]
struct Msg {
    #[serde(skip)]
    #[allow(dead_code)]
    name: String,

    #[serde(rename(serialize = "nike_name", deserialize = "nike_name"))]
    name2: String,

    #[serde(alias = "aaage3")]
    age3: i32,

    // #[serde(rename(serialize = "real_age", deserialize = "age"))]
    age2: i32,

    #[serde(default)]
    age: i32,

    #[serde(default = "age4_default")] // default must fn -> T
    age4: i32,

    #[serde(serialize_with = "age35_to_str")]
    age5: i32,

    #[serde(deserialize_with = "age35_str_to")]
    age6: i32,
}

fn age4_default() -> i32 {
    111
}

fn age35_to_str<S>(age5: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = age5.to_string();
    s.serialize(serializer)
}

fn age35_str_to<'de, D>(d: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d).unwrap();
    let j = s.parse::<i32>().unwrap();
    Ok(j)
}
