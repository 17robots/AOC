// determine whetehre there is enough tree cover here to keep a house hidden - count the number of
// trees that are visible from outside of the grid when looking directly at a row or column
// input: heightmap of each tree: height 0-9
// tree visible if all other trees around are shorter, unless on the edge, which it is visible
// check all sides to see where a tree is visible from
fn main() {
    // store as a key value pair in a hashmap with arr of which sides are visible
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut field: Vec<(usize, usize, &str)> = vec![];
    for i in 0..lines.len() {
        let chars = lines[i].split("").collect::<Vec<_>>();
        let chars = chars.iter().filter(|x| !x.is_empty()).collect::<Vec<_>>();
        for j in 0..chars.len() {
            field.push((i, j, chars[j]));
        }
    }

    // part 1
    //let visibility: usize = field.iter().map(|x| {
    //    let mut row = field.iter().filter(|y| x.0 == y.0).collect::<Vec<_>>();
    //    row.sort_by(|z,y| z.1.cmp(&y.1));

    //    let left_visible = &row[..x.1].iter().filter(|y| y.2 >= x.2 && y.1 != x.1).collect::<Vec<_>>().is_empty() == &true;
    //    if left_visible { return 1;}

    //    let right_visible = &row[x.1..].iter().filter(|y| y.2 >= x.2 && y.1 != x.1).collect::<Vec<_>>().is_empty() == &true;
    //    if right_visible { return 1;}

    //    let mut col = field.iter().filter(|y| x.1 == y.1).collect::<Vec<_>>();
    //    col.sort_by(|z,y| z.0.cmp(&y.0));

    //    let top_visible = &col[..x.0].iter().filter(|y| y.2 >= x.2 && y.0 != x.0).collect::<Vec<_>>().is_empty() == &true;
    //    if top_visible { return 1;}

    //    let bot_visible = &col[x.0..].iter().filter(|y| y.2 >= x.2 && y.0 != x.0).collect::<Vec<_>>().is_empty() == &true;
    //    if bot_visible { return 1;}

    //    return 0;
    //}).sum();
    //println!("{:?}", visibility);

    // part 2
    let visibility2: _ = field.iter().map(|x| {
        println!("{:?} run", x);
        let mut row = field.iter().filter(|y| x.0 == y.0).collect::<Vec<_>>();
        row.sort_by(|z,y| z.1.cmp(&y.1));

        println!("left");
        let mut left_total = 0;
        for tree in &row[x.1..] {
            println!("{:?}", tree);
            if tree.0 == x.0 && tree.1 == x.1 { continue; }
            if tree.2 >= x.2 {
                left_total += 1;
                break;
            }
            left_total += 1;
        }

        println!("right");
        let mut right_total = 0;
        let right = &mut row[..x.1];
        right.reverse();
        for tree in right {
            println!("{:?}", tree);
            if tree.0 == x.0 && tree.1 == x.1 { continue; }
            if tree.2 >= x.2 {
                right_total += 1;
                break;
            }
            right_total += 1;
        }

        let mut col = field.iter().filter(|y| x.1 == y.1).collect::<Vec<_>>();
        col.sort_by(|z,y| z.0.cmp(&y.0));

        println!("bot");
        let mut bot_total = 0;
        let bot = &mut col[..x.0];
        bot.reverse();
        for tree in bot {
            println!("{:?}", tree);
            if tree.0 == x.0 && tree.1 == x.1 { continue; }
            if tree.2 >= x.2 {
                bot_total += 1;
                break;
            }
            bot_total += 1;
        }

        println!("top");
        let mut top_total = 0;
        for tree in &col[x.0..] {
            println!("{:?}", tree);
            if tree.0 == x.0 && tree.1 == x.1 { continue; }
            if tree.2 >= x.2 {
                top_total += 1;
                break;
            }
            top_total += 1;
        }

        println!("{:?}, {} * {} * {} * {} = {}", x, left_total, right_total, bot_total, top_total, left_total * right_total * bot_total * top_total);
        return left_total * right_total * bot_total * top_total;
    }).max();
    println!("{:?}", visibility2);
}
