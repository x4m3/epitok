use crate::intra;

pub enum Error {
    Network,
    IntraDown,
    Parsing
}

pub struct Event {
    title: String,
    code: String,
    date: chrono::NaiveDate,
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
    // TODO: student type
}

impl Event {
    pub fn new() {}
    pub fn fetch_students() {}
}

// pub fn list_events(autologin: &str) -> Result<Vec<Event>, Error> {
pub fn list_events(autologin: &str) {
    let today = chrono::Local::today().format("%Y-%m-%d").to_string();
    let url = format!("{}/planning/load?format=json&start={}&end={}", autologin, today, today);

    // for each event that requires a token, add it and fetch list of students
}