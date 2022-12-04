// // part 1
// fn main() {
//     let text = include_str!("./input.txt").split('\n').map(|x| -> u32 {
//         if let Some(p1) = x.chars().nth(0) {
//             if let Some(p2) = x.chars().nth(2) {
//                 return match p2 {
//                     'X' => 1 + match p1 {
//                         'A' => 3,
//                         'B' => 0,
//                         'C' => 6,
//                         _ => 0
//                     },
//                     'Y' => 2 + match p1 {
//                         'A' => 6,
//                         'B' => 3,
//                         'C' => 0,
//                         _ => 0
//                     },
//                     'Z' => 3 + match p1 {
//                         'A' => 0,
//                         'B' => 6,
//                         'C' => 3,
//                         _ => 0
//                     },
//                     _ => 0,
//                 }
//             } else { return 0; }
//         } else { return 0; }
//     }).collect::<Vec<u32>>().iter().sum::<u32>();
//     println!("{}", text);
// }

fn main() {
    let text = include_str!("./input.txt").split('\n').map(|x| -> u32 {
        if let Some(p1) = x.chars().next() {
            if let Some(p2) = x.chars().nth(2) {
                match p2 {
                    'X' => match p1 {
                        'A' => 3,
                        'B' => 1,
                        'C' => 2,
                        _ => 0
                    },
                    'Y' => match p1 {
                        'A' => 1 + 3,
                        'B' => 2 + 3,
                        'C' => 3 + 3,
                        _ => 0
                    },
                    'Z' => match p1 {
                        'A' => 2 + 6,
                        'B' => 3 + 6,
                        'C' => 1 + 6,
                        _ => 0
                    },
                    _ => 0,
                }
            } else { 0 }
        } else { 0 }
    }).collect::<Vec<u32>>().iter().sum::<u32>();
    println!("{}", text);
}
