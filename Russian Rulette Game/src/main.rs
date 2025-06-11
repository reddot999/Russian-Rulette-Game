use std::io;
use rand::Rng;


fn main() {

    println!("Welcome to Russian Rulette game, please insert how many bullets you want load from 1 to 9. More bullets more danger!");
    
    // Creation bullet and loop until valid input
    let bullet: u8 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<u8>() {
            Ok(num) if num >=1 && num <= 9 => break num,
            _ => println!("Please enter a valid number of bullets (1-9)."),
        }
    };

    println!("You have a gun of 10 chambers an you load {} bullets!", bullet);

    // Randomly select a chamber to load bullets
    let mut loaded_gun = [0u8; 10]; // 0 means empty, 1 means loaded
    let mut rng =  rand::rng();
    let mut loaded = 0;
    
    while loaded < bullet {
        let chamber: usize = rng.random_range(0..10);
        if loaded_gun[chamber] == 0 {
            loaded_gun[chamber] = 1; // Load a bullet
            loaded += 1;
        }
    }
    println!("The gun is loaded, now you can start the game!");

    // Game loop
    let mut turn = 0; // 0 = player, 1 = system
    let mut current_chamber = rng.random_range(0..10);

    loop {
        let player = if turn % 2 == 0 { "You" } else { "System" };
        if player == "You" {
            println!("It's your turn!, Press enter to pull the trigger...");
        } else {
            println!("It's the system's turn!");
        }
        
        if player == "You" {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
        } else {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        if loaded_gun[current_chamber] == 1 {
            println!("Bang! {} are dead! Game over!", player);
            break;
        } else {
            println!("Click! The chamber was empty. {} survived.", player);
        }
        turn += 1;
        current_chamber = (current_chamber + 1) % 10; // Move to the next chamber
    }
    println!("Thanks for playing Russian Roulette!");
}

