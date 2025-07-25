use std::io;

fn main() {
    let RIDDLE = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut trials = 0;
    loop {
        trials += 1;
        let mut answer = String::new();
        println!("{}", RIDDLE);
       io::stdin()
       .read_line(&mut answer)
       .expect("Failed to read line");
        if answer.trim() == "The letter e" {
            println!("Number of trials: {}", trials);
            break
        }
    }
    

}
