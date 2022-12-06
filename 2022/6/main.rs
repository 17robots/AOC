use std::collections::{HashSet, VecDeque};
use std::convert::From;

#[derive(Clone)]
struct Deq(VecDeque<char>);

impl From<Deq> for HashSet<char> {
    fn from(item: Deq) -> HashSet<char> {
        let mut hm: HashSet<char> = HashSet::new();
        for i in item.0 {
            hm.insert(i);
        }
        hm
    }
}

fn main() {
    let msg: Vec<char> = include_str!("./input.txt").trim().chars().collect();
    let mut deq: Deq = Deq(VecDeque::from([]));
    for i in 0..msg.len() {
        deq.0.push_back(msg[i]);
        match deq.0.len() {
            // part 1
            // d if d > 4 => { deq.0.pop_front(); },
            d if d > 14 => { deq.0.pop_front(); },
            14 => {},
            _ => continue,
        };
        println!("deq: {:?}", deq.0);
        let set: HashSet<char> = deq.clone().into();
        // part 1
        // if set.len() == 4 {
        // part 2
        if set.len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }
}
