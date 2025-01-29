// use crate::processors::game_generator::generate_segments;
use crate::structs::{biome::Biome, segment::Segment, location::Location, party::Party};
use std::fmt::{self, write};
use serde::{Serialize, Deserialize};

/// GameState - conditions (states) that are not influenced by the conditions of the trail (biomes, segments), or the parties (wagons, people, animals).
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub start_date: String,
    pub game_length: u8,
    pub game_date: GameDate,
    pub biomes: Vec<Biome>,
    pub segments: Vec<Segment>,
    pub locations: Vec<Location>,
    pub parties: Vec<Party>,
    pub score: Vec<Player_Score>,
}

impl GameState {
    pub fn create_game() -> GameState {
        let mut test_data = GameState {
            start_date: String::from("April 15, 1842"),
            game_length: 26,
            game_date: GameDate {
                week_number: 0,
                month: String::from("April"),
            },
            biomes: Vec::new(),
            segments: Vec::new(),
            locations: Vec::new(),
            parties: Vec::new(),
            score: Vec::new(),
        };

        test_data.biomes.push(Biome {
            name: String::from("Biome Uno"),
        });
        test_data
    }

    pub fn change_state(&mut self, prop: Message) {
        match prop.action {
            ActionType::IncWeek => self.game_date.increment_week(),
        }
    }

    pub fn change_party_state(prop: Message) {}
    pub fn read_state() {}
}

impl fmt::Display for GameState {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "start_date: {} \n", self.start_date)?;
        write!(format, "game_length: {} \n", self.game_length)?;
        write!(format, "game_date.week_number: {} \n", self.game_date.week_number)?;
        write!(format, "game_date.month: {} \n", self.game_date.month)?;

        writeln!(format, "Biomes:")?;
        for biome in &self.biomes {
            writeln!(format, " {}", biome)?;
        }

        writeln!(format, "segments:")?;
        for leg in &self.segments {
            writeln!(format, " {}", leg)?;
        }

        writeln!(format, "Player_Score:")?;
        for score in &self.score {
            writeln!(format, " {}", score)?;
        }

        Ok(())
    }
}
pub struct Message {
    // state_type: StateType,
    // action_thing: String,
    // set_position: String,
    pub action: ActionType,
}

pub enum ActionType {
    IncWeek,
}

// enum StateType {
//     Party,
//     World,
//     Global,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct GameDate {
    pub week_number: u8,
    pub month: String,
}

impl GameDate {
    // GameState.GameState::increment_week();
    pub fn increment_week(&mut self) {
        self.week_number += 1;
    }
}

// score
#[derive(Serialize, Deserialize, Debug)]
pub struct Player_Score {
    pub party_name: String,
    pub position: u32,
    pub head_count: u32,
}

impl Player_Score {
    pub fn increment_position(&mut self, inc: u32) {
        self.position += inc;
    }
    pub fn decrement_position(&mut self, dec: u32) {
        self.position -= dec;
    }
    pub fn increment_head_count(&mut self, inc: u32) {
        self.head_count += inc;
    }
    pub fn decrement_head_count(&mut self, dec: u32) {
        self.head_count -= dec;
    }
}

impl fmt::Display for Player_Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player_Score.party_name: {} \n Player_Score.position: {} \n Player_Score.head_count: {} \n", self.party_name, self.position, self.head_count)
    }
}
