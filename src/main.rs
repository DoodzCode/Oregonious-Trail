mod processors;
mod structs;

extern crate chrono;

use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


use processors::conditions_processor::cycle_conditions;
use processors::report_processor::status_report;
use structs::{
    game_state::GameState,
    // biome::Biome,
    // leg::Leg,
    // location::Location,
    // party::Party,
};

//TODO come back to the question of do we need territories to be separate?

fn main() {
    let (tx, rx) = mpsc::channel();
    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!();

    // player_input == "host" {
    //     spawn host process
    // }

    let sever_thread: thread::JoinHandle<()> = thread::spawn(move || {

        let mut game_data: GameState = GameState::create_game();
       

        tx.send(&mut game_data).unwrap();
        
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
    });
    // let mut game_data: Game_Data = Game_Data::create_game();



    let game_data_ref = rx.recv().unwrap();

    status_report(game_data_ref);

    // main loop

    // shutdown
}

/*
round
    player turn
*/
