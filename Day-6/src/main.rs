use std::{
    collections::btree_map::Range,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    //Why does Rust take file paths relative to the file?
    //this comes from SRC
    let INPUT = include_str!("../input/input.txt");

    let now = Instant::now();
    task_1(INPUT);
    println!("Time elapsed: {:?}", now.elapsed());

    let now = Instant::now();
    task_2(INPUT);
    println!("Time elapsed: {:?}", now.elapsed());
}

fn task_1(input: &str) {
    let SizeOfInput = input.len();
    let regionToCheck = 4;

    for test in 0..SizeOfInput {
        let mut charsToCompare = &input[test..test + regionToCheck];

        let chars: Box<[_]> = charsToCompare.chars().collect();
        //Box = Rust's Unique pts (nobody uses them lol)
        //Forces heap memory allocations.
        let mut isSignal = true;

        for compare1 in 0..regionToCheck {
            for compare2 in compare1 + 1..regionToCheck {
                if chars[compare1] == chars[compare2] {
                    isSignal = false;
                    break;
                }
            }
        }

        if (isSignal == true) {
            println!("{}", test + regionToCheck);
            //FUCK THIS MAGIC NUMBER
            break;
        }
    }
}


fn task_2(input: &str) {
    let SizeOfInput = input.len();
    let regionToCheck = 14;

    for test in 0..SizeOfInput {
        let mut charsToCompare = &input[test..test + regionToCheck];

        let chars: Box<[_]> = charsToCompare.chars().collect();
        //Box = Rust's Unique pts (nobody uses them lol)
        //Forces heap memory allocations.
        let mut isSignal = true;

        for compare1 in 0..regionToCheck {
            for compare2 in compare1 + 1..regionToCheck {
                if chars[compare1] == chars[compare2] {
                    isSignal = false;
                    break;
                }
            }
        }

        if (isSignal == true) {
            println!("{}", test + regionToCheck);
            //FUCK THIS MAGIC NUMBER
            break;
        }
    }
}
