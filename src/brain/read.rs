use std::io::{
    stdin,
    stdout,
    Write
};

pub fn read() -> String {
    let mut input = String::new();
    let _ = stdout().flush();
    
    stdin().read_line(&mut input).expect("Unexpected error");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    return input;
}