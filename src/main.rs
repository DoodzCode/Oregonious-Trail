mod processors;
mod structs;

extern crate chrono;

use processors::conditions_processor::cycle_conditions;
//use processors::game_generator::{generate_game, generate_legs};
use processors::report_processor::status_report;
use structs::game_state::{self, GameState};

//TODO come back to the question of do we need territories to be separate?
// struct Territory {
//     name: String
// }

fn main() {

    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!();

    let mut game_state: GameState = GameState::create_game();
    status_report(&mut game_state);

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
        //* decision_controller();
        // Global Report
    }

    /*------------------------------------------------------------------------------------------- */


    /*  main recursive function

    fn game_loop(game_cycle: u8) -> u8 {
        if is_end_of_game(game_cycle) {
            return 0;
        }

        game_loop(game_cycle - 1)
    }

    fn is_end_of_game(cycle_number: u8) -> bool {
        cycle_number < 1
    }


    // let outer_var = 42;
    
    // A regular function can't refer to variables in the enclosing environment
    // fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // let closure_inferred  = |i: i32|          i + outer_var  ;

    // Call the closures.
    // println!("closure_annotated: {}", closure_annotated(1));
    // println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    // let one = || 1;
    // println!("closure returning one: {}", one());

------------------------------------------------------------------------------------------- */

    // shutdown
    GameState::close_game();

}

