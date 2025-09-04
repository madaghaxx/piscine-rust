use std::io;
fn main() {
    let mut trials = 0;
    let answer = "The letter e\n".to_string();
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        trials += 1;
        if input == answer {
            println!("Number of trials: {trials}");
            break;
        }
    }
}
