use anyhow::Result as Res;

fn main() -> Res<()> {
    let mut crates: Vec<Vec<char>> = vec![];
    let mut z = include_str!("./input.txt").split("\r\n\r\n").map(|s| s.split("\r\n").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    for i in 0..z.len() {
        match i {
            0 => {
                crates = z[0].pop().unwrap().split("").filter(|s| !s.trim().is_empty()).map(|c| c.chars().nth(0).unwrap()).map(|c| {
                    let mut v = vec![];
                    v.push(c);
                    v
                }).collect();
                let crate_len = crates.len();
                z[0].reverse();
                z[0].iter().for_each(|l| {
                    let chars: Vec<char> = l.chars().collect();
                    for i in 0..chars.len() { 
                        if chars[i] == '[' {
                            println!("{} / {} = {}",i, crate_len, i / crate_len);
                            crates[i / 4].push(chars[i + 1]);
                        }
                    }
                });
                println!("{:?}", crates);
            },
            1 => {
                z[1].iter().for_each(|l| {
                    let l: Vec<&str> = l.split(" ").filter(|s| s != &String::from("move")).collect();
                    if let [count, _, from, _, to] = l[..] {
                        // part 2
                        let count = count.parse::<u8>().unwrap();
                        let mut crane: Vec<char> = vec![];
                        for _ in 0..count {
                            let start = crates.iter_mut().find(|x| x[0].to_string() == from).unwrap();
                            let j = start.pop().unwrap();
                            crane.push(j);
                        }
                        let end = crates.iter_mut().find(|x| x[0].to_string() == to).unwrap(); 
                        crane.reverse();
                        for i in crane { end.push(i) }
                        // part 1
                        // for _ in 0..count {
                        //     let start = crates.iter_mut().find(|x| x[0].to_string() == from).unwrap();
                        //     let j = start.pop().unwrap();
                        //     let end = crates.iter_mut().find(|x| x[0].to_string() == to).unwrap();
                        //     end.push(j);
                        // }
                    } else { return;}
                });
            },
            _ => todo!(),
        };
    }
    println!("{:?}", crates.iter_mut().map(|c| c.pop().unwrap()).collect::<Vec<_>>());
    Ok(()) 
}
