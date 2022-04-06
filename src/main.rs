use std::io;

fn main() {
    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("读取失败");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match (number % 4, number % 3, number % 2) {
            (0,0,0) => println!("{} is divisible by 4, 3 and 2",number),
            (0,0,_) => println!("{} is divisible by 4 and 2",number),
            (0,_,0) => println!("{} is divisible by 4 and 3",number),
            (0,_,_) => println!("{} is divisible by 4",number),
            (_,0,0) => println!("{} is divisible by 3 and 2",number),
            (_,0,_) => println!("{} is divisible by 3",number),
            (_,_,0) => println!("{} is divisible by 2",number),
            (_,_,_) => println!("{} is not divisible by 4, 3 or 2",number),
        }
    }
}
