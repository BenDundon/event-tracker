use std::fmt;

pub struct Member {
    id: i32,
    firstname: String,
    lastname: String,
    totalHours: f32
}

impl Member {
    pub fn new(lastname: String, firstname: String) -> Self {
        Member {
            id: 0,
            firstname,
            lastname,
            totalHours: 0.0
        }
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.lastname, self.firstname)
    }
}
