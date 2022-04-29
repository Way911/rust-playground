use rand::thread_rng;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i: i32 = rand::random();
    println!("The random i32 is {}", i);

    let x: u8 = rand::random();
    println!("The random u8 is {}", x);

    let x: f64 = rand::random();
    println!("The random f64 is {}", x);

    let x: bool = rand::random();
    println!("The random bool {}", x);

    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(-10.0..=10.0);
    println!("Number from -10. to 10.: {}", y);
    println!("Number from 0 to 9: {}", rng.gen_range(0..=10));

    let mut rng = thread_rng();
    for i in 1..10 {
        println!("Random number #{}: {}", i, rng.gen_range(0..=100));
    }

    let mut arr = [0i32; 9];
    thread_rng().try_fill(&mut arr[..])?;
    println!("Random number array {:?}", arr);

    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);
    Ok(())
}
