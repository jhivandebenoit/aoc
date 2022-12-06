use std::{fs, collections::{HashSet, hash_map::RandomState}};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not parse file");

    let mut buffer = Vec::new();
    let mut pos = 0;
    let unique = 14;
    // find the unique set of 4 characters and how many exist before it;
    for c in input.chars() {

        if buffer.len() == unique{
            buffer.pop();
        }
        buffer.insert(0, c);
        if buffer.len() < unique {
            continue;
        }
        pos += 1;
        let test :HashSet<char,RandomState> = HashSet::from_iter(buffer.to_owned());
        if test.len() == unique {
            println!("Unique {} found {:?}",unique,pos);
            break;
        }
    }
}
