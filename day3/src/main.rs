use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let inp = read_input_file().unwrap();
    let num_rows = inp.len();
    println!("num_rows: {}", num_rows);
    let mut sums: Vec<usize> = vec![0; inp[0].len()];
    for row in inp {
        for (i, &bit) in row.iter().enumerate() {
            sums[i] += bit as usize
        }
    }
    let gamma_rate_binary: Vec<u8> = sums
        .iter()
        .map(|&s| if s > num_rows / 2 { 1 } else { 0 })
        .collect();
    let epsilon_rate_binary: Vec<u8> = gamma_rate_binary.iter().map(|&bit| 1 - bit).collect();

    println!("sums: {:?}", sums);
    println!("gamma_rate_binary: {:?}", gamma_rate_binary);
    println!("epsilon_rate_binary: {:?}", epsilon_rate_binary);

    let gamma_rate_decimal: u32 = binary_to_decimal(gamma_rate_binary);
    let epsilon_rate_decimal: u32 = binary_to_decimal(epsilon_rate_binary);

    println!("gamma_rate_decimal: {:?}", gamma_rate_decimal);
    println!("epsilon_rate_decimal: {:?}", epsilon_rate_decimal);

    println!("Product: {}", gamma_rate_decimal * epsilon_rate_decimal);
}
fn binary_to_decimal(bin: Vec<u8>) -> u32 {
    bin.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| b as u32 * u32::pow(2, i as u32))
        .sum()
}

fn read_input_file() -> Result<Vec<Vec<u8>>, io::Error> {
    let mut path = env::current_dir().expect("failed to get pwd");
    path.push(r"src\test_input.txt");
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        let n = line
            .trim()
            .split("")
            .filter(|x| x.len() > 0)
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        v.push(n)
    }
    Ok(v)
}
