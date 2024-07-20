use chrono::naive::NaiveDateTime;
use std::fmt;

pub struct Event {
    id: i32,
    name: String,
    date: NaiveDateTime,
    length: f32
}

impl Event {
    pub fn new(
        name: String,
        date: String,
        length: f32
    ) -> Self {
        let format = "%Y-%m-%d %H:%M";
        let dt = NaiveDateTime::parse_from_str(&date, format);
        return Event {
            id: 0,
            name,
            date: dt.expect("REASON") ,
            length
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.date)
    }
}
