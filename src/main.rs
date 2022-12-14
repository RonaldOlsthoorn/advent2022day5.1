use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

fn calculate_score(outcome: i32, opponent_play: i32)-> i32{

    // -1 for lose, 0 for draw 1 for win

    let mut diff = outcome + opponent_play;
    let mut own_play = diff;

    if own_play > 2 {
        own_play = 0;
    } else if own_play < 0 {
        own_play = 2
    }

    println!("own_play {} opponent_play {} outcome {} score: {}", own_play, opponent_play, outcome, 1 + own_play + 3 * (outcome + 1));
    1 + own_play + 3 * (outcome + 1)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut scoreMap = HashMap::new();
    scoreMap.insert('a', 1);
    scoreMap.insert('b', 2);
    scoreMap.insert('c', 3);
    scoreMap.insert('d', 4);
    scoreMap.insert('e', 5);
    scoreMap.insert('f', 6);
    scoreMap.insert('g', 7);
    scoreMap.insert('h', 8);
    scoreMap.insert('i', 9);
    scoreMap.insert('j', 10);
    scoreMap.insert('k', 11);
    scoreMap.insert('l', 12);
    scoreMap.insert('m', 13);
    scoreMap.insert('n', 14);
    scoreMap.insert('o', 15);
    scoreMap.insert('p', 16);
    scoreMap.insert('q', 17);
    scoreMap.insert('r', 18);
    scoreMap.insert('s', 19);
    scoreMap.insert('t', 20);
    scoreMap.insert('u', 21);
    scoreMap.insert('v', 22);
    scoreMap.insert('w', 23);
    scoreMap.insert('x', 24);
    scoreMap.insert('y', 25);
    scoreMap.insert('z', 26);

    scoreMap.insert('A', 27);
    scoreMap.insert('B', 28);
    scoreMap.insert('C', 29);
    scoreMap.insert('D', 30);
    scoreMap.insert('E', 31);
    scoreMap.insert('F', 32);
    scoreMap.insert('G', 33);
    scoreMap.insert('H', 34);
    scoreMap.insert('I', 35);
    scoreMap.insert('J', 36);
    scoreMap.insert('K', 37);
    scoreMap.insert('L', 38);
    scoreMap.insert('M', 39);
    scoreMap.insert('N', 40);
    scoreMap.insert('O', 41);
    scoreMap.insert('P', 42);
    scoreMap.insert('Q', 43);
    scoreMap.insert('R', 44);
    scoreMap.insert('S', 45);
    scoreMap.insert('T', 46);
    scoreMap.insert('U', 47);
    scoreMap.insert('V', 48);
    scoreMap.insert('W', 49);
    scoreMap.insert('X', 50);
    scoreMap.insert('Y', 51);
    scoreMap.insert('Z', 52);

    let mut letterBins = HashSet::new();
    let mut foundLetterTypes = HashSet::new();

    let mut score = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        println!("line: {}", l);

        for (i, c) in l.chars().enumerate(){
            if i < l.len() / 2 {
                letterBins.insert(c);
            } else{
                if letterBins.contains(&c) && !foundLetterTypes.contains((&c)) {
                    score += *scoreMap.get(&c).unwrap();
                    foundLetterTypes.insert(c);
                    println!("found character: {} in both compartments", c);
                }
            }
        }
        letterBins.clear();
        foundLetterTypes.clear();
    }

    println!("total score {}", score);
}