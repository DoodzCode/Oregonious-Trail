/// struct for legs (segments) of the trail
#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub distance: i32,  // 80
    // pub guide_note: String, // "west by north to Kansas River Crossing"
}

impl Segment {
    pub fn create(name: &str, distance: i32) -> Segment {
        Segment {
            name: String::from(name),
            distance,
            // guide_note,
        }
    }

    //TODO add a generator function
}