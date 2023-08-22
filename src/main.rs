use npcs::{Alignment as nature, Character as Char, Weapons as arms};
use std::collections::HashMap;

mod npcs;

fn main() {
    let villager_a = Char {
        name: String::from("Cornelus Fairwether"),
        age: 6400,
        village: String::from("Pumpkinpatch"),
        mother: None,
        father: None,
        primary_weapon: arms::ShortWords,
        aligment: nature::Evil,
    }; //interesting, couldnt define outside of
       // String with String::from, string literal would say do type &str which would be a reference to a string

    println!("{:#?}", villager_a);
    println!("Hello, world!");

    let mut hm: HashMap<String, i32> = HashMap::new();

    hm.insert(String::from("Fatma"), 10);

    println!("{:#?}", hm);

    let some_text: i32= match "nonum".parse(){
            Ok(num) => num,
            Err(error) => {
                println!("{}", error);
                0
            }
    };



}
