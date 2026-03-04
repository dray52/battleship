/*
By: Draydon Levesque
Date: 2026-02-05
Program Details: Battleship
*/
use rand::Rng;
use core::num;
use std::io;
fn main() {
    // create a vector of strings for the placements of the ships and the guesses
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
    // temporary list to draw the map
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
    // variables to keep track of the game state
    let mut input = String::new();
   
    let mut tries = 0;
    let mut win = 0;
    let mut game = true;
    let mut play = false;
    let mut ships = -1;
let mut num_ships = false;
    // main game loop
    while game {
        println!(
            "Enter 1 to play
Enter 2 to exit"
        );
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // reset the game state
        if input.trim() == "1" {
            placements2 = vec![
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
            guesses = vec![
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
            ];num_ships=true;
            while num_ships{
                
            
             println!(
            "Enter 1 for 2 ships
Enter 2 for 3 ships"
        );
         let mut shipps = String::new();
        io::stdin().read_line(&mut shipps).expect("Failed to read line");
            // generate 2 ship placements
            
            if shipps.trim() == "1" {
                shipgenerator(&mut placements, &mut placements2);
                shipgenerator(&mut placements, &mut placements2);
                ships = 4;
                num_ships=false;
            }
            // generate 3 ship placements
            else if shipps.trim() == "2" {
            shipgenerator(&mut placements, &mut placements2);
            shipgenerator(&mut placements, &mut placements2);
            shipgenerator(&mut placements, &mut placements2);
            ships = 6;
            num_ships=false;

            }
            else{
println!("Invalid input");
            }}
            // draw the map
            println!(" /  1    2    3    4    5");
            draw_map(&mut temp, &mut guesses);
            play = true;
            input.clear();
            tries = 0;
        }
        // exit the game
        else if input.trim() == "2" {
            println!("Goodbye!");
            game = false;
        } else {
            println!("Invalid input");
        }
        //Loops the game so you can keep guessing until you win or lose
        while play {
            let mut guess = String::new();
            println!("Take a guess");

            io::stdin().read_line(&mut guess).expect("Failed to read line");

            // check if the guess is a hit or a miss and update the game state accordingly
            if placements2.iter().any(|p| p == guess.trim()) {
                for i in 0..placements2.len() {
                    if guesses[i] == "X" && placements2[i] == guess.trim() {
                        println!("You already hit that spot!");
                    } else if placements2[i] == guess.trim() {
                        guesses[i] = "X".to_string();
                        println!(" /  1    2    3    4    5");
                        draw_map(&mut temp, &mut guesses);
                        println!("You Hit!");
                        tries += 1;
                    }
                }
            } else if placements.iter().any(|p| p == guess.trim()) {
                for i in 0..placements.len() {
                    if guesses[i] == "O" && placements[i] == guess.trim() {
                        println!("You already missed that spot!");
                    } else if placements[i] == guess.trim() {
                        guesses[i] = "O".to_string();
                        println!(" /  1    2    3    4    5");
                        draw_map(&mut temp, &mut guesses);
                        println!("You missed!");
                        tries += 1;
                    }
                }
            }
            println!("tries: {}", tries);
            // check if the player has won or lost
            if tries > 30 {
                println!("You lose! it took you {} tries!", tries);
                play = false;
            }
            for i in 0..placements2.len() {
                // if 4 X's are in the guesses vector, the player wins
                if guesses[i] == "X" {
                    win += 1;
                }
            }
            if win == ships {
                println!("You win! it took you {} tries!", tries);
                play = false;
            } else {
                // reset the win counter for the next guess
                win = 0;
            }
        }
    }
}

fn shipgenerator(placements: &mut Vec<String>, placements2: &mut Vec<String>) {
    // generate 2 ship placements
    let mut rng = rand::rng();
    let mut dice = rng.random_range(0..placements.len());
    // check if the placement is already taken and if it is, generate a new one until it isn't
    while placements[dice] == placements2[dice] {
        dice = rng.random_range(0..placements.len());
    }
    // move the String out of `placements` into `placements2` without borrowing
    placements2[dice] = placements[dice].clone();
    // loops through the first 5 placements and checks if the dice roll is equal to the index, if it is, it generates a random number between 1 and 3 to determine where the other part of the ship will be placed, then it checks if the placement is valid and if it is, it moves the String from `placements` to `placements2` without borrowing
    for i in 0..5 {
        if dice == i {
            let mut otherplace = rng.random_range(1..4);
            if (i == 1 && otherplace == 1) || (i == 4 && otherplace == 3) {
                otherplace = rng.random_range(1..3);
                if otherplace == 1 && i == 1 {
                    otherplace = dice + 1;
                } else if otherplace == 1 && i == 4 {
                    otherplace = dice - 1;
                } else {
                    otherplace = dice + 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            } else {
                if otherplace == 1 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice + 5;
                } else if otherplace == 3 {
                    otherplace = dice + 1;
                }

                placements2[otherplace] = placements[otherplace].clone();
            }
        }
    }
    // loops through the next 5 placements and checks if the dice roll is equal to the index, if it is, it generates a random number between 1 and 4 to determine where the other part of the ship will be placed, then it checks if the placement is valid and if it is, it moves the String from `placements` to `placements2` without borrowing
    for i in 5..10 {
        if dice == i {
            let mut otherplace = rng.random_range(1..5);
            if (i == 5 && otherplace == 1) || (i == 9 && otherplace == 3) {
                otherplace = rng.random_range(1..4);
                if otherplace == 1 && i == 5 {
                    otherplace = dice + 1;
                } else if otherplace == 3 && i == 9 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice + 5;
                } else {
                    otherplace = dice - 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            } else {
                if otherplace == 1 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice - 5;
                } else if otherplace == 3 {
                    otherplace = dice + 1;
                } else if otherplace == 4 {
                    otherplace = dice + 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            }
        }
    }
    // loops through the next 5 placements and checks if the dice roll is equal to the index, if it is, it generates a random number between 1 and 4 to determine where the other part of the ship will be placed, then it checks if the placement is valid and if it is, it moves the String from `placements` to `placements2` without borrowing
    for i in 10..15 {
        if dice == i {
            let mut otherplace = rng.random_range(1..4);
            if (i == 10 && otherplace == 1) || (i == 14 && otherplace == 3) {
                otherplace = rng.random_range(1..4);
                if otherplace == 1 && i == 10 {
                    otherplace = dice + 1;
                } else if otherplace == 3 && i == 14 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice + 5;
                } else {
                    otherplace = dice - 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            } else {
                if otherplace == 1 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice - 5;
                } else if otherplace == 3 {
                    otherplace = dice + 1;
                } else if otherplace == 4 {
                    otherplace = dice + 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            }
        }
    }
    // loops through the next 5 placements and checks if the dice roll is equal to the index, if it is, it generates a random number between 1 and 4 to determine where the other part of the ship will be placed, then it checks if the placement is valid and if it is, it moves the String from `placements` to `placements2` without borrowing
    for i in 15..20 {
        if dice == i {
            let mut otherplace = rng.random_range(1..4);
            if (i == 15 && otherplace == 1) || (i == 19 && otherplace == 3) {
                otherplace = rng.random_range(1..4);
                if otherplace == 1 && i == 15 {
                    otherplace = dice + 1;
                } else if otherplace == 3 && i == 19 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice + 5;
                } else {
                    otherplace = dice - 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            } else {
                if otherplace == 1 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice - 5;
                } else if otherplace == 3 {
                    otherplace = dice + 1;
                } else if otherplace == 4 {
                    otherplace = dice + 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            }
        }
    }
    // loops through the next 5 placements and checks if the dice roll is equal to the index, if it is, it generates a random number between 1 and 4 to determine where the other part of the ship will be placed, then it checks if the placement is valid and if it is, it moves the String from `placements` to `placements2` without borrowing
    for i in 20..25 {
        if dice == i {
            let mut otherplace = rng.random_range(1..4);
            if (i == 20 && otherplace == 1) || (i == 24 && otherplace == 3) {
                otherplace = rng.random_range(1..3);
                if otherplace == 1 && i == 20 {
                    otherplace = dice + 1;
                } else if otherplace == 1 && i == 24 {
                    otherplace = dice - 1;
                } else {
                    otherplace = dice - 5;
                }
                placements2[otherplace] = placements[otherplace].clone();
            } else {
                if otherplace == 1 {
                    otherplace = dice - 1;
                } else if otherplace == 2 {
                    otherplace = dice - 5;
                } else if otherplace == 3 {
                    otherplace = dice + 1;
                }
                placements2[otherplace] = placements[otherplace].clone();
            }
        }
    }
}

fn draw_map(temp: &mut Vec<String>, guesses: &mut Vec<String>) {
    // draws the first 5
    for i in 0..5 {
        temp.push(guesses[i].clone());
    }
    println!("A {:?}", temp);
    temp.clear();
    // draws the next 5
    for i in 5..10 {
        temp.push(guesses[i].clone());
    }
    println!("B {:?}", temp);
    temp.clear();
    // draws the next 5
    for i in 10..15 {
        temp.push(guesses[i].clone());
    }
    println!("C {:?}", temp);
    temp.clear();
    // draws the next 5
    for i in 15..20 {
        temp.push(guesses[i].clone());
    }
    println!("D {:?}", temp);
    temp.clear();
    // draws the next 5
    for i in 20..25 {
        temp.push(guesses[i].clone());
    }
    println!("E {:?}", temp);
    temp.clear();
}
