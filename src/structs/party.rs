use core::fmt;

#[derive(Debug)]
pub struct Party {
    pub name: String,
    position: u16,
    pub head_count: u16,
    // pub wagons: Vec<Wagon>,
}

impl Party {
    pub fn create(name: &str) -> Party {
        Party {
            name: String::from(name),
            position: 0,
            head_count: 140,
        }
    }

    pub fn generate_test_parties() -> Vec<Party> {
        let mut parties: Vec<Party> = Vec::new();
        parties.push(Party::create("Red Party"));
        parties.push(Party::create("Green Party"));
        parties.push(Party::create("Blue Party"));
        parties
    }

    pub fn increment_position(&self, distance: u16) -> u16 {
        self.position + distance
    }

    pub fn decrement_position(&self, distance: u16) -> u16 {
        self.position - distance
    }

    pub fn increment_head_count(&self, amount: u16) -> u16 {
        self.head_count + amount
    }

    pub fn decrement_head_count(&self, amount: u16) -> u16 {
        self.head_count - amount
    }

    pub fn give_position(&self) -> &u16 {
        &self.position
    }
}

impl fmt::Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Party: {}   Head Count: {}  Position: {}", self.name, self.head_count, self.position)
    }
}