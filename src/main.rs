/*
By: Draydon Levesque
Date: 2026-02-05
Program Details: Battleship
*/
use rand::Rng;
use std::io;
fn main() {
    let mut placements = vec![
        "A1".to_string(),
        "A2".to_string(),
        "A3".to_string(),
        "A4".to_string(),
        "A5".to_string(),
        "B1".to_string(),
        "B2".to_string(),
        "B3".to_string(),
        "B4".to_string(),
        "B5".to_string(),
        "C1".to_string(),
        "C2".to_string(),
        "C3".to_string(),
        "C4".to_string(),
        "C5".to_string(),
        "D1".to_string(),
        "D2".to_string(),
        "D3".to_string(),
        "D4".to_string(),
        "D5".to_string(),
        "E1".to_string(),
        "E2".to_string(),
        "E3".to_string(),
        "E4".to_string(),
        "E5".to_string(),
    ];
    let mut placements2 = vec![
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
    ];
    let mut temp = vec![];

    let mut guesses = vec![
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
        ".".to_string(),
    ];

    let mut input = String::new();
    
    let mut game = true;
    println!(
        "Enter 1 to play
Enter 2 to exit"
    );
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() == "1" {
        shipgenerator(&mut placements, &mut placements2);
        println!("{:?}", placements);
    } else if input.trim() == "2" {
        println!("Goodbye!");
    } else {
        println!("Invalid input");
    }
    println!(" /  1    2    3    4    5");
    draw_map(&mut temp, &mut guesses);
while game{
    let mut guess = String::new();
    println!("Take a guess");

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("input {}", guess);
    println!("placements {:?}", placements);
    if placements2.iter().any(|p| p == guess.trim()) {
        for i in 0..placements2.len() {
            if placements2[i] == guess.trim() {
                guesses[i] = "X".to_string();
                println!(" /  1    2    3    4    5");
                draw_map(&mut temp, &mut guesses);
            }
        }println!("You Hit!");

        
    } else if placements.iter().any(|p| p == guess.trim()) {
        for i in 0..placements.len() {
            if placements[i] == guess.trim() {
                guesses[i] = "O".to_string();
                println!(" /  1    2    3    4    5");
                draw_map(&mut temp, &mut guesses);
            }
        }println!("You missed!");
        
    }}
}

fn shipgenerator(placements: &mut Vec<String>, placements2: &mut Vec<String>) {
    let mut rng = rand::rng();
    let dice = rng.random_range(0..placements.len());

    // move the String out of `placements` into `placements2` without borrowing
    placements2[dice] = placements[dice].clone();

    println!("dice {}", placements[dice]);

    println!("{}", dice);
}

fn draw_map(temp: &mut Vec<String>, guesses: &mut Vec<String>) {
    for i in 0..5 {
        temp.push(guesses[i].clone());
    }
    println!("A {:?}", temp);
    temp.clear();
    for i in 5..10 {
        temp.push(guesses[i].clone());
    }
    println!("B {:?}", temp);
    temp.clear();
    for i in 10..15 {
        temp.push(guesses[i].clone());
    }
    println!("C {:?}", temp);
    temp.clear();
    for i in 15..20 {
        temp.push(guesses[i].clone());
    }
    println!("D {:?}", temp);
    temp.clear();
    for i in 20..25 {
        temp.push(guesses[i].clone());
    }
    println!("E {:?}", temp);
    temp.clear();
}
