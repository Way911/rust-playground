pub fn pattern_test1() {
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite color, {color}, as the background!");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    let x = 3;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub fn pattern_test2() {
    let p = Point { x: 3, y: 4 };
    let Point { x: a, y: b } = p;
    println!("({a},{b})");

    let Point { x, y } = p;
    println!("({x},{y})");

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis ({x},{y})"),
    }
}

struct Point3D<T>(T, T, T);

pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
pub enum Message {
    Quit,
    Hello { id: i32 },
    Move { x: i16, y: i16 },
    Write(String),
    ChangeColor(Color),
}

pub fn pattern_test3() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}");
        }
        _ => (),
    }

    let color = Color::Rgb(0, 160, 255);
    if let Color::Rgb(r, g, b) = color {
        println!("red {r}, green {g}, and blue {b}")
    }

    let point3d = Point3D(0, 0, 0);

    let Point3D(x, y, z) = point3d;
    println!("({x},{y},{z})");
}

pub fn pattern_test4() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let number = (2, 4, 8, 16, 32);
    match number {
        (first, .., last) => println!("first: {first}, last: {last}"),
    }
    match number {
        (first, _, third, _, last) => println!("first: {first}, third: {third}, last: {last}"),
    }
}

pub fn pattern_test5() {
    let s = Some(String::from("Hello"));

    // if let Some(_s) = s { //这里会报错，借用了s，不能再次使用s
    //     println!("found a string");
    // }

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}

pub fn pattern_test6() {
    let p = Point3D(1, 2, 3);
    match p {
        Point3D(x, ..) => println!("x: {x}"),
    }

    match p {
        Point3D(x, .., y) => println!("x: {x}, y: {y}"),
    }
}

pub fn pattern_test7() {
    let num = Some(7);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

pub fn pattern_test8() {
    let x = 1;
    let y = true;
    match x {
        1 | 2 | 3 if y => println!("yes"),
        _ => println!("no"),
    }
}

pub fn pattern_test9() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
        _ => (),
    }
}
