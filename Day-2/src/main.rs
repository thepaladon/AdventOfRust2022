use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input/input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    //task_1(buf_reader);
    task_2(buf_reader);
    
    println!("Hello, world!");
}

fn task_2(buf_reader: BufReader<File>) {
    let mut Score = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let CharLineIterator = line.chars().collect::<Vec<_>>();

        let EnemyInput = CharLineIterator[0] as i32 - 65;
        let GameOutcome = CharLineIterator[2] as i32 - 88;
        let mut YourInput = -1;
        // 0 = Rock, 1 = Paper, 2 = Scissors
        // 0 = Loss, 1 = Draw, 2 = Win
        // 0 -> 2  = -2 //win
        // 1 -> 0  = 0
        // 2 - > 1 = 1

        //WIN 
        if (GameOutcome == 2) {
            match EnemyInput {
                0 => {YourInput = 1}
                1 => {YourInput = 2}
                2 => {YourInput = 0}
                _ => {}
            }
        } else if (GameOutcome == 0) { //LOSS
            match EnemyInput {
                0 => {YourInput = 2}
                1 => {YourInput = 0}
                2 => {YourInput = 1}
                _ => {}
            }
        }else{
            YourInput = EnemyInput;
        }

        let FinalTruce = YourInput - EnemyInput;

        if EnemyInput == YourInput {
            Score += 3;
        } //Check for draw
        if FinalTruce == -2 || FinalTruce == 1 {
            Score += 6;
        } //Check for win
        if FinalTruce == -1 || FinalTruce == 2 {
            Score += 0;
        } //Check for loss

        Score += YourInput + 1;
        println!("Score: {}", Score);
    }
}

fn task_1(buf_reader: BufReader<File>) {
    let mut Score = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let CharLineIterator = line.chars().collect::<Vec<_>>();

        let EnemyInput = CharLineIterator[0] as i32 - 65;
        let YourInput = CharLineIterator[2] as i32 - 88;
        // 0 = Rock, 1 = Paper, 2 = Scissors
        // 0 -> 2  = -2 //win
        // 1 -> 0  = 0
        // 2 - > 1 = 1

        // 0 -> 1 = -1 //loss
        // 1 -> 2 = -1
        // 2 -> 0 = 2

        println!("Input: {}, {}", EnemyInput, YourInput);
        let FinalTruce = YourInput - EnemyInput;

        if EnemyInput == YourInput {
            Score += 3;
        } //Check for draw
        if FinalTruce == -2 || FinalTruce == 1 {
            Score += 6;
        } //Check for win
        if FinalTruce == -1 || FinalTruce == 2 {
            Score += 0;
        } //Check for loss

        Score += YourInput + 1;
        println!("Score: {}", Score);
    }
}
