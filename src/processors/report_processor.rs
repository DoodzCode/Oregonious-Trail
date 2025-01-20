//use crate::structs::game_data::Game_Data;
use crate::structs::game_state::GameState;



pub fn status_report(game_state: &mut GameState) {
    println!("Status Report: ");
    println!("{:?}", game_state);

    println!();
    println!();
}