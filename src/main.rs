use std::{sync::mpsc, thread};

fn main() {
    let (sender, reciever) = mpsc::channel();
    let sender_temp = sender.clone();
    loop {
        let sender_temp = sender_temp.clone();
        let mut val = String::new();
        std::io::stdin().read_line(&mut val).unwrap();
        println!("val is {}", val);
        if val == "exit!\n" {
            println!("exiting...");
            thread::spawn(move || {
                sender_temp.send(String::from("chat has ended")).unwrap();
            });
            let recieved = reciever.recv();
            println!("recieved: {}", recieved.unwrap());
            break;            
        }
        else{
            thread::spawn(move || {
                sender_temp.send(val).unwrap();
            });
        }

        let recieved = reciever.recv();
        println!("recieved: {}", recieved.unwrap())
    }
}
