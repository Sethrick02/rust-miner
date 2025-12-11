use rand::Rng;
use std::io::{self, Write};

// This holds all data about the player's progress & resources
struct GameState {
    iron_ore: u64,
    money: u64,
}

// Implementation block for the GameState struct, defining its associated functions/methods
impl GameState {
    // Defines the starting state for a new game
    fn new() -> Self {
        GameState {
            iron_ore: 0,
            money: 0,
        }
    }

    // Game method for the 'mine' action
    // Takes a mutable reference because it modifies the iron_ore field
    // Returns the amount of iron ore mined
    fn mine(&mut self) -> u64 {
        // Random number generator
        let mut rng = rand::thread_rng();

        // Generate random number between 1-3
        let mined_amount: u64 = rng.gen_range(1..=3);

        // Updates the game state by adding the mined amount to the iron_ore field
        self.iron_ore += mined_amount;
        println!("--- Mining in progress... ---");
        // Returns the amount of iron ore mined
        mined_amount
    }
}

fn main() {
    // Initializes the game state. Is mutable because it will be modified during gameplay
    let mut game_state = GameState::new();

    // Main game loop. Runs indefinitely until the player uses the 'quit' command
    loop {
        // Status Display
        println!("\n--- Status ---");
        println!("Iron Ore: {}", game_state.iron_ore);
        println!("Money: {}", game_state.money);
        println!("--------------");

        // Prompt & Flush
        // Prints the command prompt without a newline
        print!("> ");
        // Forces the printed content to display immediately
        io::stdout().flush().expect("Failed to flush");

        // Creates a mutable string to store the user input
        let mut input = String::new();

        // Reads the user input
        // The '.expect()' handles the result type, crashing the program if I/O fails
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Cleans up the input
        let command = input.trim().to_lowercase();

        // Process Command
        // Uses a match expression to handle different user commands
        match command.as_str() {
            // Help Command
            "help" => {
                println!("Available commands:");
                println!("  mine - Mine iron ore");
                println!("  sell - Sell iron ore");
                println!("  quit - Quit the game");
            }

            // Mine Command
            "mine" => {
                let mined_amount = game_state.mine();
                println!("You mined {} iron ore!", mined_amount);
            }

            // Sell Command
            "sell" => {
                // Constants define the amount of ore to sell and the price per ore
                const SELL_AMOUNT_ORE: u64 = 5;
                const PRICE_PER_ORE: u64 = 3;
                // Calculate the total money gain from selling the ore
                let total_money_gain = SELL_AMOUNT_ORE * PRICE_PER_ORE;

                // Check if the player has enough ore to sell
                if game_state.iron_ore >= SELL_AMOUNT_ORE {
                    // Subtract the sold ore from the player's inventory & update the money gained
                    game_state.iron_ore -= SELL_AMOUNT_ORE;
                    game_state.money += total_money_gain;
                    println!(
                        "You sold {} iron ore for {} money!",
                        SELL_AMOUNT_ORE, total_money_gain
                    );
                } else {
                    // Transaction failed due to insufficient ore
                    println!("You need at least {} iron ore to sell!", SELL_AMOUNT_ORE);
                }
            }
            // Quit Command
            "quit" => {
                println!("--- Quitting game... ---");
                // The break statement exits the loop
                break;
            }
            // Catch all for unrecognized commands
            _ => {
                println!("Invalid command: '{}'. Try 'mine' or 'quit'.", command);
            }
        }
    }
}
