mod processors;
mod controllers;
mod structs;
mod utils;

extern crate chrono;

use processors::conditions_processor::cycle_conditions;
use processors::report_processor::status_report;
use controllers::decision_controller::{party_to_delay, party_to_proceed};
// use controllers::decision_controller::party_to_delay;
// use controllers::decision_controller::party_to_proceed;

use structs::game_state::GameState;
use utils::{d20, save_to_file, load_game_from_file, get_input};

//TODO come back to the question of do we need territories to be separate?

fn main() {
    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!();

    let mut game_state: GameState = load_game_from_file("src/config/game_state.json").expect("Failed to load game data");

    status_report(&mut game_state);

    println!("{:?}", &game_state);

    // main loop
    loop {
        if game_state.game_date.week_number > game_state.game_length - 1 {
            break;
        }
        cycle_conditions(&mut game_state);
        // user prompt
        //* decision_controller();
        //cycle actions
        //* actions_processor();
        // user prompt - go or no go.
        for party in &mut game_state.parties {
            println!("{:?} do you want to 1. proceed or 2. delay?", party.name);
            let cmd: String = get_input();     
            match cmd.as_str() {
                "1" => party_to_proceed(party),
                "2" => party_to_delay(party),       
                _ => println!("Invalid Response")
            } 
        }
        //* decision_controller();
        // Global Report
    }

    // shutdown
}

/*
round
    player turn
*/
