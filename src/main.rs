use std::io::{self, Write};

struct GameState {
    iron_ore: u64,
    money: u64,
}

impl GameState {
    fn new() -> Self {
        GameState {
            iron_ore: 0,
            money: 0,
        }
    }
}

fn main() {
    let mut game_state = GameState::new();

    loop {
        // Status Display
        println!("\n--- Status ---");
        println!("Iron Ore: {}", game_state.iron_ore);
        println!("Money: {}", game_state.money);
        println!("--------------");

        // Prompt & Flush
        print!("> ");
        io::stdout().flush().expect("Failed to flush");

        // Read Input
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim().to_lowercase();

        // Process Command
        match command.as_str() {
            "mine" => {
                game_state.iron_ore += 1;
                println!("--- Mining in progress... ---");
            }
            "quit" => {
                println!("--- Quitting game... ---");
                break;
            }
            _ => {
                println!("Invalid command: '{}'. Try 'mine' or 'quit'.", command);
            }
        }
    }
}
