use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut highestValue = 0;
    let mut elfHighestValue = 0;

    let mut vector = Vec::new();

    for line in buf_reader.lines() {
        // ? sign at line discards the error, I guess?
        // -> std::io::Result<()>. If error, the entire program errors lol
        let value = line?.parse::<i32>();

        //Option 1: Rust's If Statement
        if let Ok(value) = value {
            elfHighestValue += value;
        } else if let Err(_) = value {
            if (highestValue < elfHighestValue) {
                highestValue = elfHighestValue;
            }
            println!("Value: {elfHighestValue}");
            println!("High Score: {highestValue}");
            vector.push(elfHighestValue);
            elfHighestValue = 0;
        }

        //Option 2: Rust's Switch Statements
        // match value {
        //     Ok(value) => {}

        //     Err(_) => {
        //         if (highestValue < elfHighestValue) {
        //             highestValue = elfHighestValue;
        //         }
        //         println!("Value: {elfHighestVal}");
        //         println!("High Score: {highestValue}");
        //         elfHighestValue = 0;
        //     }
        // }
    }

    let fattestElf = vector.len() - 1;
    println!("Num Elves {}", vector.len());
    
    vector.sort();
    
    println!("Biggest Elves {} ", vector[fattestElf- 2] + vector[fattestElf - 1] + vector[fattestElf]);

    Ok(())
}
