use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, receiver) = mpsc::channel();

    let sender1 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread", "hi", "from", "the", "thread"];
        for val in vals {
            sender1.send(format!("1:{val}")).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let sender2 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread", "hi", "from", "the", "thread"];
        for val in vals {
            sender2.send(format!("2:{val}")).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for val in receiver {
        println!("{val}");
    }
}
