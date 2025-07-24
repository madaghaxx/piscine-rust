use std::io;
fn main() {
    let correct_answer = "The letter e";
    let mut answer = String::new();
    let mut trials = 0;
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        io::stdin().read_line(&mut answer).expect("walo");
        trials += 1;
        if answer.trim() == correct_answer {
            println!("Number of triels: {}", trials);
            break;
        }
    }
}
