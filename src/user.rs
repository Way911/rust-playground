use std::error::Error;
use std::io::{ErrorKind, Write};
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
        let mut file = create_file(filename)?;
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
    fn persist(&self, filename: &str) -> Result<(), Box<dyn Error>>;
    fn load(filename: &str) -> Result<T, Box<dyn Error>>;
}

impl BincodeSerde<Self> for User {
    fn persist(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = create_file(filename)?;
        // let data = match bincode::serialize_into(file, self) {
        //     Ok(it) => Ok(0),
        //     Err(err) => Err(err),
        // };
        let data = bincode::serialize_into(file, self)?;
        Ok(data)
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        // let mut data = String::new();
        // file.read_to_string(&mut data)?;
        let user = bincode::deserialize_from(file)?;
        Ok(user)
    }
}

fn create_file(filename: &str) -> Result<File, io::Error> {
    let file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                let path = std::path::Path::new(filename);
                let prefix = path.parent().unwrap();
                std::fs::create_dir_all(prefix)?;
                File::create(filename)?
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    Ok(file)
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
