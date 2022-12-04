fn main() {
    let _stuff: u16 = include_str!("./input.txt")
        .lines().map(|x| {
            let range_num = x.split(",").flat_map(|y| y.split("-").map(|z| z.parse::<u16>().unwrap())).collect::<Vec<_>>();
            let range1 = range_num[1] - range_num[0];
            let range2 = range_num[3] - range_num[2];

            println!("{}..{}, range: {}; {}..{}, range: {}", range_num[0], range_num[1], range1, range_num[2], range_num[3], range2);

            if range1 > range2 {
                // if range_num[0] <= range_num[2] && range_num[1] >= range_num[3] {    
                if (range_num[0] <= range_num[2] && range_num[2] <= range_num[1]) || (range_num[0] <= range_num[3] && range_num[3] <= range_num[1]) {
                    return 1;
                } else { 
                    return 0;
                }
            }
            // part 1
            // if range_num[2] <= range_num[0] && range_num[3] >= range_num[1] {
            if (range_num[2] <= range_num[0] && range_num[0] <= range_num[3]) || (range_num[2] <= range_num[1] && range_num[1] <= range_num[3]) {
                return 1;
            } else { 
                return 0; 
            }
        }).sum();
    println!("{}", _stuff);
}
