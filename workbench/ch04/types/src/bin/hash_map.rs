use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();
    capitals.insert("Japan", "Tokyo");
    capitals.insert("UK", "London");
    capitals.insert("US", "Washington D.C.");

    let targets = vec!["Japan", "Korea", "UK", "US"];

    for tg in targets {
        // match capitals.get(tg) {
        match capitals.get(&tg) {
            // keyが&strなのに&&str出なくても良いのはなぜ？
            Some(capital) => println!("The capital of {} is {}", tg, *capital),
            None => println!("{} not found", tg),
        }
    }

    let mut map = HashMap::new();
    map.insert(1, "1");
    map.insert(2, "2");

    let targets = vec![1, 2, 3];

    for tg in targets {
        match map.get(&1) {
            Some(x) => println!("The capital of {} is {}", tg, x),
            None => println!("{} not found", tg),
        }
    }
}
