// https://www.rust-lang.org/learn/get-started
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();

    let mut message = String::from("Hello, ");
    message.push('R');
    message.push_str("ust!");
    
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}