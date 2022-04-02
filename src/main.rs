
use std::{io, cmp::Ordering::{Less,Greater,Equal}};

use rand::{thread_rng, Rng};
fn main() {
    println!("猜数字游戏");
    let mut rng = thread_rng();
    let secret_num = rng.gen_range(1..101);
    loop {
        println!("请输入你猜的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数字是{}", guess);
        match guess.cmp(&secret_num) {
            Less => println!("你猜的数字小于答案"),
            Greater => println!("你猜的数字大于答案"),
            Equal => {
                println!("你猜对了");
                break;
            }
        }
    }
}
