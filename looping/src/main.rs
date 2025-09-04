use std::io;
fn main() {
    loop {
        let mut trials = 1;
        let answer = "The letter e\n".to_string();
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input == answer {
            println!("Number of trials: {trials}");
            break;
        }
        trials += 1;
    }
}
