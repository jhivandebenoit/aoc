use std::fs;
fn main() {
    let input = fs::read_to_string("test.txt").expect("Couldn't parse input");

    // Vec of vectors containing 3 lines
    let group = input
        .split("\n")
        .take(3)
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", group)
}
