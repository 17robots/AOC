// depth first search no
// part 1
// const SIZE_THRESHOLD: usize = 100000;
const ALLOWED_OCCUPYABLE_SIZE: usize = 70000000 - 30000000;
fn main() { 
    // part 1
    // let mut total_below = 0;
    type Folder = (&'static str, usize);
    let mut folders: Vec<Folder> = vec![];
    let mut calculated_folders: Vec<Folder> = vec![];
    let commands: Vec<&str> = include_str!("./input.txt").lines().filter(|x| !x.contains("$ ls")).collect();
    // push folders on cd
    // pop on .. and then add total to the next last folder (check to see if it meets the threshold
    // and add to the total)
    // if file found, add size to last folder in arr
    for command in commands {
        let pieces: Vec<&str> = command.split(" ").collect();
        match pieces[0] {
           "$" => {
               match pieces[2] {
                   ".." => {
                       let last_folder = folders.pop().unwrap();
                       // part 1
                       // if last_folder.1 < SIZE_THRESHOLD { total_below += last_folder.1; }
                       folders.last_mut().unwrap().1 += last_folder.1;

                       // part 2 - includes above
                       calculated_folders.push(last_folder);
                   },
                   _ => {
                       folders.push((pieces[2], 0));
                   },
               }
           },
           "dir" => {},
           _ => {
               let file_size = pieces[0].parse::<usize>().unwrap();
               folders.last_mut().unwrap().1 += file_size;
           },
        }
    }
    // part 2
    loop {
        if folders.is_empty() {break;}
        let last_folder = folders.pop().unwrap();
        if last_folder.0 != "/" {
            folders.last_mut().unwrap().1 += last_folder.1;
        }
        calculated_folders.push(last_folder);
    }
    if ALLOWED_OCCUPYABLE_SIZE > calculated_folders.last().unwrap().1 {println!("No files need removed"); return;}

    let mut calculated_folders = calculated_folders.iter().filter(|x| x.1 > calculated_folders.last().unwrap().1 - ALLOWED_OCCUPYABLE_SIZE).collect::<Vec<_>>();
    calculated_folders.sort_by(|x,y| x.1.cmp(&y.1));
    println!("{:?}", calculated_folders[0].1);

    // part 1
    // println!("total size: {}", total_below);
}
