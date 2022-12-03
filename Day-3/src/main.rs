use std::{
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
    let mut Score = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let test = line.split_at(line.len() / 2);
        //println!("Size {}, Data: {} || {}", line.len(), test.0, test.1);

        let mut similarChars: Vec<char> = Vec::new();
        for char1 in test.0.chars() {
            for char2 in test.1.chars() {
                if char1 == char2 {
                    if (!similarChars.contains(&char1)) {
                        similarChars.push(char1);
                        let mut letterValue = 0;
                        letterValue = char1 as i32;
                        if (char1.is_lowercase()) {
                            letterValue -= 96;
                        }
                        if (char1.is_uppercase()) {
                            letterValue -= 64;
                            letterValue += 26;
                        }
                        //println!("Char: {}, Value: {} ", char1, letterValue);
                        Score += letterValue;
                    }
                }
            }
        }
    }
    println!("Final Result {}", Score);
}

fn task_2(buf_reader: BufReader<File>) {
    let mut Score = 0;
    let mut iter = 0;

    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();

    for line in buf_reader.lines() {
        let line = line.unwrap();
        group.push(line);
        iter += 1;

        if iter == 3 {
            iter = 0;
            groups.push(group.clone());
            group.clear();
        }
    }

    let mut canOnlyWriteOnce = true;
    for ThreeSet in groups {
        assert!(ThreeSet.len() == 3);
        canOnlyWriteOnce = true;
        for char1 in ThreeSet[0].chars() {
            for char2 in ThreeSet[1].chars() {
                for char3 in ThreeSet[2].chars() {
                    if char1 == char2 && char1 == char3 {
                        let mut letterValue = 0;
                        letterValue = char1 as i32;
                        if (char1.is_lowercase()) {
                            letterValue -= 96;
                        }
                        if (char1.is_uppercase()) {
                            letterValue -= 64;
                            letterValue += 26;
                        }
                        //println!("Char: {}, Value: {} ", char1, letterValue);
                        if canOnlyWriteOnce {Score += letterValue; canOnlyWriteOnce = false;}
                    }
                }
            }
        }
    }

    println!("Final Result {}", Score);
}
