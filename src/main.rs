/*
By: Draydon Levesque
Date: 2026-02-05
Program Details: Battleship
*/
use rand::Rng;
use std::io;
fn main() {
    let mut placements = vec![
        "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3",
        "E4", "E5",
    ];
    let mut placements2 = vec![
        ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
    ];
    let mut temp = vec![];

    let mut guesses = vec![
        ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
    ];

    let mut input = String::new();
    let mut guess = String::new();
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
draw_map( &mut temp, &mut guesses);
    
    println!("Take a guess");

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("input {}", guess);
    println!("placements {:?}", placements);
    if placements.contains(&guess.trim()) {
        for i in 0..placements.len() {
            if placements[i] == guess.trim() {
                guesses[i] = "O";
               draw_map(&mut temp, &mut guesses);
            }
        }println!("You missed!");
        
    } else {
        for i in 0..placements.len() {
            if placements[i] == guess.trim() {
                guesses[i] = "X";
               draw_map(&mut temp, &mut guesses);
            }
        }
        println!("You Hit!");
    }
}

fn shipgenerator(placements: &mut Vec<&str>, placements2: &mut Vec<&str>) {
    let mut rng = rand::rng();
    let dice: usize = rng.random_range(0..placements.len());
    let mut otherplace: i32 =5;
    placements2[dice] = placements[dice];
  placements.remove(dice);
 
    if dice as i32 - otherplace > 0 {
        otherplace = -5;
    }
    //placements.remove((dice - otherplace) as usize);

    println!("{}", dice);
}

fn draw_map(temp: &mut Vec<String>, guesses: &mut Vec<&str>) {
 for i in 0..5 {
            temp.push(guesses[i].to_string());
        }
        println!("A {:?}", temp);
        temp.clear();
        for i in 5..10 {
            temp.push(guesses[i].to_string());
        }
        println!("B {:?}", temp);
        temp.clear();
        for i in 10..15 {
            temp.push(guesses[i].to_string());
        }
        println!("C {:?}", temp);
        temp.clear();
        for i in 15..20 {
            temp.push(guesses[i].to_string());
        }
        println!("D {:?}", temp);
        temp.clear();
        for i in 20..25 {
            temp.push(guesses[i].to_string());
        }
        println!("E {:?}", temp);
        temp.clear();

}