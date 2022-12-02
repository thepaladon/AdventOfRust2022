use std::{fs::File, io::{BufReader, BufRead}};



fn main() {
    let file = File::open("input/input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut Score = 0;
    for line in buf_reader.lines(){

        let line = line.unwrap();
        let CharLineIterator = line.chars().collect::<Vec<_>>();  

        let EnemyInput = CharLineIterator[0] as i32 - 65;
        let YourInput = CharLineIterator[2] as i32 - 88;
        // 0 = Rock, 1 = Paper, 2 = Scissors
        // 0 -> 2  = -2
        // 1 -> 0  = 0
        // 2 - > 1 = 1

        // 0 -> 1 = -1
        // 1 -> 2 = -1 
        // 2 -> 0 = 2
    
        println!("Input: {}, {}", EnemyInput , YourInput);
        let FinalTruce = YourInput - EnemyInput ;

        if EnemyInput == YourInput { Score += 3;} //Check for draw
        if FinalTruce == -2 || FinalTruce == 1 { Score += 6;} //Check for win
        if FinalTruce == -1 || FinalTruce == 2 { Score += 0;} //Check for loss

        Score += YourInput + 1;
        println!("Score: {}", Score);


    }

    println!("Hello, world!");
}
