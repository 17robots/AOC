use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<u32> = Vec::new();
    let mut total_cals: u32 = 0;
    if let Ok(lines) = read_lines("./elves.txt") {
        for line in lines {
            if let Ok(cals) = line {
                match cals.parse::<u32>() {
                    Ok(i) => total_cals = total_cals + i,
                    Err(_) => {
                        elves.push(total_cals);
                        total_cals = 0;
                    }
                }
            }
        }
    }
    // part 1
    elves.sort();
    elves.reverse();
    let top_3: u32 = elves.iter().take(3).sum();
    println!("{}", top_3);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}

