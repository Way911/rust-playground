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

    pub fn persist(&self, filename: &str) -> Result<usize, io::Error> {
        let mut file = File::create(filename)?;
        // let data = match bincode::serialize_into(file, self) {
        //     Ok(it) => Ok(0),
        //     Err(err) => Err(err),
        // };
        let data = bincode::serialize(self).unwrap();
        file.write_all(&data)?;
        Ok(data.len())
    }

    pub fn load(filename: &str) -> Result<Self, io::Error> {
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
    use super::*;

    #[test]
    fn user_persist_and_load() {
        let user1 = User::default();
        user1.persist("./user1.bc").unwrap();
        let user2 = User::load("./user1.bc").unwrap();
        assert_eq!(user1, user2);
        // println!("{user1:?}");
    }
}
