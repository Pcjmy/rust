use std::collections::HashMap;

fn main() {
    let mut transcript: HashMap<&str, u32> = HashMap::new();

    transcript.insert("alice", 95);
    transcript.insert("bob", 92);

    match transcript.get(&"alice") {
        Some(data) => println!("alice's score is {:?}", data),
        None => println!("alice is not in the transcript"),
    }

    match transcript.get(&"jack") {
        Some(data) => println!("jack's score is {:?}", data),
        None => println!("jack is not in the transcript"),
    }

    transcript.remove(&"alice");

    match transcript.get(&"alice") {
        Some(data) => println!("alice's score is {:?}", data),
        None => println!("alice is not in the transcript"),
    }

    for (&name, &score) in transcript.iter() {
        println!("{}'s score is {}", name, score);
    }
}
