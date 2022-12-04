fn main() {
    // part 1
    let _sacks: usize = include_str!("./test.txt").split("\n").map(|sack| sack).map(|sack| {
        let s1 = &sack[0..sack.len()/2];
        let s2 = &sack[sack.len()/2..]; 
        for c1 in s1.chars() {
            for c2 in s2.chars() {
                if c1 == c2 {
                    if (c1 as i32 - 'a' as i32) < 0 {
                        return c1 as usize - 'A' as usize + 1 + 26;
                    }
                    return c1 as usize - 'a' as usize + 1;
                }
            }
        }
        return 0;
    }).collect::<Vec<usize>>().iter().sum();

    // part 2
    let sacks2: Vec<&str> = include_str!("./test.txt").split("\n").collect::<Vec<&str>>();
    let sacks3 = sacks2.chunks(3).map(|s| s.into()).collect::<Vec<&[&str]>>(); 
    let sum: usize = sacks3.iter().map(|group| {
        for c1 in group[0].chars() {
            for c2 in group[1].chars() {
                for c3 in group[2].chars() {
                    if c1 == c2 && c2 == c3 {
                        if (c1 as i32 - 'a' as i32) < 0 {
                            return c1 as usize - 'A' as usize + 1 + 26;
                        }
                        return c1 as usize - 'a' as usize + 1;
                    }
                }
            }
        }
        return 0;
    }).collect::<Vec<usize>>().iter().sum();
    println!("{}", sum);
}

