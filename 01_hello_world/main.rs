use ferris_says::says;
use std::io::{stdout, BufWriter};


fn main(){
    let stdout = stdout();
    let message = String::from("Hello fellow rustaceans!")
    let width = message.chars().count();


    let mut write = BufWriter::new(stdout.lock());
    says(message.as_bytes(), width, &mut write).unwrap();




}