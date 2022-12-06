use std::{
    collections::btree_map::Range,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let file = File::open("input/input.txt").expect("file not found!");
    let mut buf_reader = BufReader::new(file);

    let now = Instant::now();
    //task_1(buf_reader);
    println!("Time elapsed: {:?}", now.elapsed());

    let now = Instant::now();
    task_2(buf_reader);
    println!("Time elapsed: {:?}", now.elapsed());
}

fn task_1(buf_reader: BufReader<File>) {
    let mut TheStacks: Vec<Vec<char>> = Vec::new();
    let mut instructionsIter = 0;
    let mut instructions: Vec<Vec<i32>> = Vec::new();

    TheStacks.resize(9, Vec::new());

    let mut CraneInput = true;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let words = line.chars();

        let mut i = 0;

        if CraneInput {
            for word in words {
                i += 1;
                let i = i / 4;
                if (word.is_alphabetic()) {
                    TheStacks[i].push(word);
                    println!("Letter {} in {} column", word, i);
                }
            }
        }

        if !CraneInput {
            instructions.push(Vec::new());
            for word in line.split_whitespace() {
                let instruction_val = word.parse::<i32>();

                if let Ok(val_as_int) = instruction_val {
                    instructions[instructionsIter].push(val_as_int);
                }
            }
            instructionsIter += 1;
        }

        if (line.is_empty()) {
            CraneInput = false;
        };
    }

    //check column
    // for column in input {
    //     for value in column {
    //         print!("{}", value);
    //     }
    //     println!("");
    // }

    println!("Steps time!");
    for oneLine in instructions {
        assert!(oneLine.len() == 3, "{}", oneLine.len());

        let mut capacity: i32 = oneLine[0];
        let stack1: i32 = oneLine[1];
        let stack2: i32 = oneLine[2];

        if (capacity > TheStacks[stack1 as usize - 1].len().try_into().unwrap()) {capacity = TheStacks[stack1 as usize - 1 ].len() as i32; }

        
        let slice = &TheStacks[stack1 as usize - 1 ].clone()[0..capacity as usize];
        TheStacks[stack1 as usize - 1].drain(0..capacity as usize);

        for char in slice{
            TheStacks[stack2 as usize - 1].insert(0, *char); 
            print!("{}", char);        
        }

        println!("");   
    }

    println!("Movement process finished");   

    // for column in TheStacks {
    //     for value in column {
    //         print!("{}", value);
    //     }
    //     println!("");
    // }

    println!("Your final output is:");

    for column in TheStacks {
        print!("{}", column.first().unwrap())
    }
    
    println!();
}


fn task_2(buf_reader: BufReader<File>) {
    let mut TheStacks: Vec<Vec<char>> = Vec::new();
    let mut instructionsIter = 0;
    let mut instructions: Vec<Vec<i32>> = Vec::new();

    TheStacks.resize(9, Vec::new());

    let mut CraneInput = true;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let words = line.chars();

        let mut i = 0;

        if CraneInput {
            for word in words {
                i += 1;
                let i = i / 4;
                if (word.is_alphabetic()) {
                    TheStacks[i].push(word);
                    println!("Letter {} in {} column", word, i);
                }
            }
        }

        if !CraneInput {
            instructions.push(Vec::new());
            for word in line.split_whitespace() {
                let instruction_val = word.parse::<i32>();

                if let Ok(val_as_int) = instruction_val {
                    instructions[instructionsIter].push(val_as_int);
                }
            }
            instructionsIter += 1;
        }

        if (line.is_empty()) {
            CraneInput = false;
        };
    }

    println!("Steps time!");
    for oneLine in instructions {
        assert!(oneLine.len() == 3, "{}", oneLine.len());

        let mut capacity: i32 = oneLine[0];
        let stack1: i32 = oneLine[1];
        let stack2: i32 = oneLine[2];

        if (capacity > TheStacks[stack1 as usize - 1].len().try_into().unwrap()) {capacity = TheStacks[stack1 as usize - 1 ].len() as i32; }

        
        let slice = &TheStacks[stack1 as usize - 1 ].clone()[0..capacity as usize];
        TheStacks[stack1 as usize - 1].drain(0..capacity as usize);

        let mut iter = 0;
        for char in slice{
            TheStacks[stack2 as usize - 1].insert(iter, *char);
            iter += 1;
            print!("{}", char);        
        }

        println!("");   
    }

    println!("Movement process finished");   

    // for column in TheStacks {
    //     for value in column {
    //         print!("{}", value);
    //     }
    //     println!("");
    // }

    println!("Your final output is:");

    for column in TheStacks {
        print!("{}", column.first().unwrap())
    }
    
    println!();
}