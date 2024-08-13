use serde_json::json;

fn main() {
    let hananko = json!({
        "name": "hananko",
        "age": 20,
        "is_student": true,
        "hobbies": ["reading", "programming", "watching movies"],
        "address": {
            "country": "Japan",
            "city": "Tokyo"
        }
    });

    println!("hanako debug: {:#?}", hananko);

    println!("hanako [\"name\"]: {}", hananko["name"]);
    println!(
        "hanako [\"name\"].as_str().unwrap(): {}",
        hananko["name"].as_str().unwrap()
    );

    let mut taro = json!({});
    taro["name"] = json!("taro");
    taro["age"] = json!(30);
    // FIXME: "1" to 1, string keyでないとダメみたい？
    taro["1"] = json!("1");
    taro["address"] = json!({
        "country": "Japan",
        "city": "Osaka"
    });
    println!("taro debug: {:#?}", taro);

    let mut members = json!({});
    members["members"] = json!([hananko, taro]);
    println!("members debug: {:#?}", members);
    println!("JSON: {}", members.to_string());
}
