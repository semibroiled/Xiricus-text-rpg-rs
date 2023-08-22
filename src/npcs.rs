// name to mod.rs in npcs or npcs.rs in src. I dont know why it has to be mod?
//just covered in 7.5 with mod its old style

#[derive(Debug)]
pub enum Weapons {
    HandAxe,
    ShortSwords,
    ShortWords,
    CommonSense,
}

#[derive(Debug)]
pub enum Alignment {
    Evil,
    Neutral,
    Good,
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub age: i64,
    pub village: String,
    pub mother: Option<String>,
    pub father: Option<String>, //to break Character recursion. Box<Character> instead of Character idk what it does
    pub primary_weapon: Weapons,
    pub aligment: Alignment,
}
