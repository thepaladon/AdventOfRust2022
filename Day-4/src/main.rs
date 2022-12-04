use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant, collections::btree_map::Range,
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
        let assigned_grp: Vec<&str> = line.split(",").collect();

        let mut Range1: Vec<i32> = Vec::new();
        let mut Range2: Vec<i32> = Vec::new();
        for elf in assigned_grp {
            let zone_range: Vec<&str> = elf.split("-").collect();
            assert!(zone_range.len() == 2); //no more than 2 variables

            let range1 = zone_range[0].parse::<i32>().unwrap();
            let range2 = zone_range[1].parse::<i32>().unwrap();

            if Range1.len() == 0 {
                for i in range1..range2 + 1{
                    Range1.push(i);
                }
            }else {
                for i in range1..range2 + 1{
                    Range2.push(i);
                }
            }
        }

        let mut matches = false;
        let Range1Lenght = Range1.len();
        let Range2Lenght = Range2.len();
        
        let mut match_counter = 0;
        for y in &Range1{
            for  x in &Range2{
                if(*y == *x){
                    match_counter += 1;
                }
            }

            if match_counter < 1 { break; } //if none match, we can continue;
            if match_counter == Range1Lenght { matches = true; break;  } //if all from 1 match, we can continue
        }

        match_counter = 0;

        for y in &Range2{
            for  x in &Range1{
                if(*y == *x){
                    match_counter += 1;
                }
            }

            if match_counter < 1 { break; } //if none match, we can continue;
            if match_counter == Range2Lenght { matches = true; break;  } //if all from 1 match, we can continue
        }
        

        if(matches) {Score += 1;}        
    }
    println!("Score {} ", Score);

}


fn task_2(buf_reader: BufReader<File>) {

    let mut Score = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let assigned_grp: Vec<&str> = line.split(",").collect();

        let mut Range1: Vec<i32> = Vec::new();
        let mut Range2: Vec<i32> = Vec::new();
        for elf in assigned_grp {
            let zone_range: Vec<&str> = elf.split("-").collect();
            assert!(zone_range.len() == 2); //no more than 2 variables

            let range1 = zone_range[0].parse::<i32>().unwrap();
            let range2 = zone_range[1].parse::<i32>().unwrap();

            if Range1.len() == 0 {
                for i in range1..range2 + 1{
                    Range1.push(i);
                }
            }else {
                for i in range1..range2 + 1{
                    Range2.push(i);
                }
            }
        }

        let mut matches = false;
        let Range1Lenght = Range1.len();
        let Range2Lenght = Range2.len();
        
        let mut match_counter = 0;
        for y in &Range1{
            for  x in &Range2{
                if(*y == *x){
                    match_counter += 1;
                }
            }

            if match_counter <= Range1Lenght && match_counter  != 0 { matches = true; break;  } //if all from 1 match, we can continue
        }

        match_counter = 0;

        for y in &Range2{
            for  x in &Range1{
                if(*y == *x){
                    match_counter += 1;
                }
            }

            if match_counter <= Range2Lenght && match_counter != 0 { matches = true; break;  } //if all from 1 match, we can continue
        }
        

        if(matches) {Score += 1;}        
    }
    println!("Score {} ", Score);

}
