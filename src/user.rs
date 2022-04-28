use std::io::Write;
use std::{fs::File, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }
}

pub trait JsonSerde<T> {
    fn persist(&self, filename: &str) -> Result<usize, io::Error>;
    fn load(filename: &str) -> Result<T, io::Error>;
}

impl JsonSerde<Self> for User {
    fn persist(&self, filename: &str) -> std::result::Result<usize, std::io::Error> {
        let json = serde_json::to_vec(self)?;
        let mut file = File::create(filename)?;
        file.write_all(&json)?;
        Ok(json.len())
    }
    fn load(filename: &str) -> std::result::Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        let user = serde_json::from_reader(file)?;
        Ok(user)
    }
}

pub trait BincodeSerde<T> {
    fn persist(&self, filename: &str) -> Result<usize, io::Error>;
    fn load(filename: &str) -> Result<T, io::Error>;
}

impl BincodeSerde<Self> for User {
    fn persist(&self, filename: &str) -> Result<usize, io::Error> {
        let mut file = File::create(filename)?;
        // let data = match bincode::serialize_into(file, self) {
        //     Ok(it) => Ok(0),
        //     Err(err) => Err(err),
        // };
        let data = bincode::serialize(self).unwrap();
        file.write_all(&data)?;
        Ok(data.len())
    }

    fn load(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        // let mut data = String::new();
        // file.read_to_string(&mut data)?;
        let user = bincode::deserialize_from(file).unwrap();
        Ok(user)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: Default::default(),
            age: Default::default(),
            gender: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Unspecified
    }
}

#[cfg(test)]
mod user_tests {

    use crate::user::User;

    #[test]
    fn user_persist_and_load_bincode() {
        use crate::user::BincodeSerde;

        let mut user1 = User::default();
        user1.name = "Tommy".into();
        user1.persist("./tmp/user1.bc").unwrap();
        let user2 = User::load("./tmp/user1.bc").unwrap();
        assert_eq!(user1, user2);
        // println!("{user1:?}");
    }

    #[test]
    fn user_persist_and_load_json() {
        use crate::user::JsonSerde;

        let mut user1 = User::default();
        user1.name = "Tommy".into();
        user1.persist("./tmp/user1.json").unwrap();
        let user2 = User::load("./tmp/user1.json").unwrap();
        assert_eq!(user1, user2);
        // println!("{user1:?}");
    }
}
