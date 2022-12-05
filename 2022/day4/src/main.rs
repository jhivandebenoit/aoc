use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't parse input");

    let lines = input.split("\n");

    // take lines and split them into each schedule
    let mut total = 0;
    for line in lines {
        let mut schedules = Vec::new();
        for pair in line.split(",") {
            println!("pair: {pair}");
            if !pair.is_empty() {
                schedules.push(
                    pair.split("-")
                        .map(|n| n.parse::<i32>().expect("Failed to parse number"))
                        .collect::<Vec<i32>>(),
                );
            }
            println!("{:?}", schedules);
        }
        if !schedules.is_empty() {
            let second = schedules.pop().unwrap();
            let first = schedules.pop().unwrap();

            let mut range1 = Vec::new();
            for x in first[0]..(first[1] + 1) {
                range1.push(x);
            }

            println!("range1 = {:?}", range1);
            let mut range2 = Vec::new();
            for x in second[0]..(second[1] + 1) {
                range2.push(x);
            }
            println!("range2 = {:?}", range2);
            let mut flag1 = true;
            for x in range1.to_owned() {
                if !range2.contains(&x) {
                    flag1 = false;
                    break;
                }
            }
            if flag1 {
                println!("range1 is contained by range2");
                total += 1;
                continue;
            }
            let mut flag1 = true;
            for x in range2 {
                if !range1.contains(&x) {
                    flag1 = false;
                    break;
                }
            }
            if flag1 {
                println!("range2 is contained by range1");
                total += 1;
            }
        }
    }
    println!("Total overlap: {}", total);
}
