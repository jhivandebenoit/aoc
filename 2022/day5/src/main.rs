use std::{collections::VecDeque, fs, io::Split, str::Chars};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't parse input");

    // let columns = (input.to_owned().split_once("\n").unwrap().0.chars().collect::<Vec<char>>().len()+1)/4;
    // println!("{columns}");
    let mut stacks :   Vec<VecDeque<char>>= Vec::new();
    for x in 0..9 {
        stacks.push(VecDeque::new());
    }
    println!("{}",stacks.len());
    for line in input.split("\n") {
        // [A] [C] [D]
        // we wanna get i where i is the index of each of the 9 stacks
        if line.len() == 0 {
            continue;
        }
        if line.contains("[") {
            for (i,c) in line.chars().enumerate().filter(|(i, v)| (i - 1) % 4 == 0) {
                let stack = (i+1)/4;
               &stacks[stack].push_back(c);
            }
        }
        if line.contains("move") {
            let mut elems = line.split(" ");
            let numb = elems.nth(1).unwrap().parse::<i32>().unwrap();
            let from = elems.nth(1).unwrap().parse::<usize>().unwrap()-1;
            let to = elems.nth(1).unwrap().parse::<usize>().unwrap()-1;
            println!("move {} from {} to {}",numb,from,to);
            for x in 0..numb {
                let val = &stacks[from].pop_back().unwrap();
                &stacks[to].push_back(*val);
            }
        }
    }
    let mut top = Vec::new();

    for x in &stacks {
    top.push(x.back().unwrap());
}
    println!("{:?}",top);
}
