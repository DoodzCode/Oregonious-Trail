use crate::structs::game_data::Game_Data;
// use crate::structs::game_data::Game_Data;
// use crate::structs::biome::Biome;
use crate::structs::segment::Leg;
use crate::structs::party::Party;

pub fn generate_game() -> Game_Data {

    let mut parties = generate_parties();
    let mut game_data: Game_Data = Game_Data::create_game();
    

}

pub fn generate_parties() -> Vec<Party> {
    let mut parties: Vec<Party> = Vec::new();

    parties.push(Party{
        name: String::from("Red Team"),
        position: 0,
    });

    parties.push(Party{
        name: String::from("Blue Team"),
        position: 0,
    });

    parties.push(Party{
        name: String::from("Green Team"),
        position: 0,
    });

    parties
}


pub fn generate_legs() -> Vec<Leg> {
    let mut legs: Vec<Leg> = Vec::new();
        
    legs.push(Leg{
        name: String::from("Leg-1"),
        // destination: String::from("Kansas River crossing"),
        distance: 150,
        guide_note: String::from("West by north about 5 or 6 days to Kansas River crossing."),
    });
    
    legs.push(Leg{
        name: String::from("Leg-2"),
        // destination: String::from("Platte River"),
        distance: 150,
        guide_note: String::from("Northwest, about 5 days to the Platte River."),
    });
    
    legs.push(Leg{
        name: String::from("Leg-3"),
        // destination: String::from("Fort Kearney"),
        distance: 150,
        guide_note: String::from("Continue up south side of the Platte to Fort Kearney"),
    });
    
    legs.push(Leg{
        name: String::from("Leg-4"),
        // destination: String::from("Fort Laramie"),
        distance: 150,
        guide_note: String::from("Follow the north fork of the Platte about 8 days to Fort Laramie."),
    });

    legs
}


// for leg in &legs { println!("ref {}, destination {}, distance {}, guide_note {}", leg.name, leg.destination, leg.distance, leg.guide_note );}