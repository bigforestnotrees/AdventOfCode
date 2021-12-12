use std::fs;

fn main() {
    let count: Vec<u32> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!(),
                })
                .collect::<Vec<u32>>()
        })
        .reduce(|a, b| a.iter().zip(b.iter()).map(|(&x, &y)| x + y).collect())
        .unwrap()
        .iter()
        .map(|count| if *count >= 500 { 1 } else { 0 })
        .collect();

    println!("{:?}", count)
}
