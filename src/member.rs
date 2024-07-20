use std::fmt;

pub struct Member {
    id: i32,
    first_name: String,
    last_name: String,
    total_hours: f32
}

impl Member {
    pub fn new(lastName: String, firstName: String) -> Self {
        Member {
            id: 0,
            first_name: firstName,
            last_name: lastName,
            total_hours: 0.0
        }
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.last_name, self.first_name)
    }
}
