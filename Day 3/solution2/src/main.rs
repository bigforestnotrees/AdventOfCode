use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::slice;

fn main() {
    // let mut counts : Vec<Vec<u32>> = vec![vec![0, 2]; 12];
    let count : Vec<u32> = {
        let lines : io::Lines<_> = read_lines("../input.txt").unwrap();
        let lines_vector : Vec<String> = lines.map(|line| line.unwrap()).collect::<Vec<String>>();
        let lines_count_vector : Vec<Vec<u32>> = lines_vector.into_iter().map(|line| line.char_count('1')).collect();
        let counts = lines_count_vector.into_iter().reduce(|a, b| a.add(b)).unwrap();
        counts.iter().map(|count| if *count >= 500 { 1 } else {0}).collect()
    };

    println!("{:?}", count)

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

trait Addable {
    fn add(&self, other: Self) -> Self;
}

impl Addable for Vec<u32> {
    fn add(&self, other: Vec<u32>) -> Vec<u32> {
        let mut new_vec : Vec<u32> = Vec::new();
        for (v1, v2) in self.iter().zip(other) {
            new_vec.push((*v1)+v2);
        }
        new_vec
    }
}

trait Countable {
    fn char_count(&self, character : char) -> Vec<u32>;
}

impl Countable for String {
    fn char_count(&self, character: char) -> Vec<u32> {
        let mut count : Vec<u32> = Vec::new();
        for (i, c) in self.chars().into_iter().enumerate() {
            if i + 1 > count.len() {
                count.push(0);
            }
            
            if c == character {
                count[i] += 1;
            }
        
        }
        count
    }
}