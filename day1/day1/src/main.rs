use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    // Part 2
    let nums = read_input_file().unwrap(); // Make main return a Result and don't be lazy by unwrapping
    let ct: i32 = (3..nums.len())
        .map(|i| if &nums[i] > &nums[i - 3] { 1 } else { 0 })
        .sum();
    println!("{}", ct);
}

/* fn main() {
    // Part 1
    let nums = read().unwrap();
    let ct: i32 = (1..nums.len())
        .map(|i| if &nums[i] > &nums[i - 1] { 1 } else { 0 })
        .sum();
    println!("{}", ct);
}
*/
fn read_input_file() -> Result<Vec<i64>, io::Error> {
    let mut path = env::current_dir().expect("failed to get pwd");
    path.push(r"src\input.txt");
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        let n = line.trim().parse().unwrap(); // Could do error mapping here
        v.push(n)
    }
    Ok(v)
}
