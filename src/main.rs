use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

// This holds all data about the player's progress & resources
#[derive(Serialize, Deserialize)]
struct GameState {
    iron_ore: u64,
    money: u64,
    workers: u64,
}

// Implementation block for the GameState struct, defining its associated functions/methods
impl GameState {
    // Defines the starting state for a new game
    fn new() -> Self {
        GameState {
            iron_ore: 0,
            money: 0,
            workers: 0,
        }
    }
    fn apply_passive_gain(&mut self) {
        if self.workers > 0 {
            const WORKER_ORE_RATE: u64 = 1;
            let passive_gain = self.workers * WORKER_ORE_RATE;
            self.iron_ore += passive_gain;
        }
    }
    fn mine(&mut self) -> u64 {
        // 1. Calculate Base Random Gain (1-3)
        let mut rng = rand::thread_rng();
        let base_gain: u64 = rng.gen_range(1..=3); // Clear name: base_gain

        // 2. Calculate Worker Efficiency Bonus (Task 9 Logic)
        // Integer division correctly applies the bonus:
        // 0-4 workers = 0 bonus; 5-9 workers = 1 bonus; 10-14 workers = 2 bonus, etc.
        const WORKERS_PER_BONUS: u64 = 5;
        let worker_bonus = self.workers / WORKERS_PER_BONUS;

        // 3. Calculate Total Mined Amount
        let total_mined_amount = base_gain + worker_bonus; // Clear name: total_mined_amount

        // 4. Update the game state with the total amount (ONLY ONCE)
        self.iron_ore += total_mined_amount;

        println!("--- Mining in progress... ---");

        // Return the total amount mined
        total_mined_amount
    }

    // Save method
    fn save_game(&self, filename: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(filename, json)?;
        Ok(())
    }

    // Load method
    fn load_game(filename: &str) -> Result<Self, io::Error> {
        // 1. Read the file content into a String. Uses ? to return io::Error on failure.
        let json = fs::read_to_string(filename)?;

        // 2. Deserialize the JSON string back into a GameState struct.
        // serde_json::from_str returns a Result<GameState, serde_json::Error>.
        // We use map_err to convert the serde_json::Error (e) into an io::Error.
        let game_state: GameState = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // 3. Return the successfully loaded GameState.
        Ok(game_state)
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
        println!("Workers: {}", game_state.workers);
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
                println!("  hire - Hire a worker");
            }

            // Mine Command
            "mine" => {
                game_state.apply_passive_gain();
                let mined_amount = game_state.mine();
                println!("You mined {} iron ore!", mined_amount);
            }

            // Sell Command
            "sell" => {
                // Constants define the amount of ore to sell and the price per ore
                game_state.apply_passive_gain();
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
                        "You sold {} iron ore for ${}!",
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
            // Hire Worker Command
            "hire" => {
                // Constants define the cost of hiring a worker
                const HIRE_COST: u64 = 50;
                // Check if the player has enough money to hire a worker
                if game_state.money >= HIRE_COST {
                    game_state.apply_passive_gain();
                    // Subtract the cost from the player's money & update the number of workers
                    game_state.money -= HIRE_COST;
                    game_state.workers += 1;
                    println!("You hired a worker for ${}!", HIRE_COST);
                } else {
                    // Transaction failed due to insufficient money
                    println!("You need at least ${} to hire a worker!", HIRE_COST);
                }
            }
            // Catch all for unrecognized commands
            _ => {
                println!(
                    "Invalid command: '{}'. Type 'help' for a list of commands.",
                    command
                );
            }
        }
    }
}
