use std::error::Error;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, Box<dyn Error>> {
        if value < 1 || value > 100 {
            return Err("数字必须在1-100之间")?;
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
