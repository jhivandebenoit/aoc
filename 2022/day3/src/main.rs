use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
};
fn main() {
    let input = fs::read_to_string("test.txt").expect("Couldn't parse input");

    // Vec of vectors containing 3 lines
    // let group = input
    //     .split("\n")
    //     .take(3)
    //     .map(|l| l.to_string())
    //     .collect::<Vec<String>>();
    // println!("{:?}", group)
    let mut sum = 0;

    for line in input.split("\n") {
        if !line.is_empty() {
            let half = line.len() / 2;
            let (left, right) = (&line[0..half], &line[half..]);
            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());

            let duplicate = left.intersection(&right);

            let duplicate = duplicate.into_iter().next();

            println!("duplicate : {:?}", duplicate);
            sum += get_points(*duplicate.expect("this wasn't an int"));
        }
    }
    println!("sum: {sum}");
}

fn get_points(v: char) -> i32 {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut point_map: HashMap<char, i32> = HashMap::new();
    let mut count = 1;
    for letter in letters.chars() {
        point_map.insert(letter, count);
        count += 1;
    }
    return point_map[&v];
}
