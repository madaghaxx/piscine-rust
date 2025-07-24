use std::io;

fn main() {
    let mut trials = 0;
    loop {
        let mut answer = String::new();
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        match io::stdin().read_line(&mut answer) {
            Ok(_n) => {
                trials += 1;
                if answer.trim() == "The letter e" {
                    println!("Number of trials: {}", trials);
                    break;
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
}
