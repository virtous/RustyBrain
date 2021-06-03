pub mod brain;
use brain::tokenizer::tokenizer;
use brain::read::read;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Welcome to RustyBrain 0.1.0");
    println!("RustyBrain is a simple BrainF*ck interpreter written in Rust");
    println!("Type 'ctrl+c' to exit");

    loop {
        let input = read();
        let token = tokenizer(input);

        if token.len() != 0 {
            println!("{:?}", token);
        }
    }
}