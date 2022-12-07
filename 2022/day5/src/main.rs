use std::{fs, io::Split, str::Chars};

fn main() {
    let input = fs::read_to_string("test.txt").expect("Couldn't parse input");

    let columns = (input.to_owned().split_once("\n").unwrap().0.chars().collect::<Vec<char>>().len()+1)/4;
    println!("{columns}");
    for line in input.split("\n") {
        // [A] [C] [D]
       for c in line.chars().enumerate().filter(|c| ) 
            
        
    }
}
