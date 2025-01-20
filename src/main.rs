mod processors;
mod structs;

extern crate chrono;

use processors::conditions_processor::cycle_conditions;
use processors::report_processor::status_report;
use structs::{
    game_data::Game_Data,
    // biome::Biome,
    // leg::Leg,
    // location::Location,
    // party::Party,
};

//TODO come back to the question of do we need territories to be separate?

fn main() {
    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!();

    // let mut game_data: Game_Data = Game_Data::create_game();
    let mut game_data: Game_Data = Game_Data::create_game();

    println!("STATUS REPORT");
    println!();
    status_report(&mut game_data);

    println!("MAIN LOOP START");
    // main loop
    loop {
        if game_data.game_date.week_number > game_data.game_length - 1 {
            break;
        }
        cycle_conditions(&mut game_data);
        // user prompt
        //* decision_controller();
        //cycle actions
        //* actions_processor();
        // user prompt - go or no go.
        //* decision_controller();
        // Global Report
    }
    println!("MAIN LOOP END");
    println!("Weeks Elapsed: {:?}", game_data.game_date.week_number);

    // shutdown
}

/*
round
    player turn
*/
