use std::{error, fmt};
use crate::intra;
use crate::student::{Student, fetch_students};

#[derive(Debug)]
pub struct Event {
    code: String,
    title: String,
    module: String,
    date: chrono::NaiveDate,
    start: String,
    end: String,
    students: Vec<Student>,
}

impl Event {
    pub fn new() {}
    pub fn fetch_students() {}
}

#[derive(Debug)]
pub enum Error {
    EventURL,
    Title,
    Module,
    Time(Time),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::EventURL => "Event doesn't have a url (how is this even possible?)",
            Error::Title => "This event does not have a title",
            Error::Module => "This event does not belong to a module",
            Error::Time(Time::Start) => "This event does not have a starting time",
            Error::Time(Time::End) => "This event does not have a finish time",
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum Time {
    Start,
    End,
}

fn parse_time(json: &serde_json::Value, time: Time) -> Option<String> {
    let time = match time {
        Time::Start => "start",
        Time::End => "end",
    };

    return match json[time].as_str() {
        Some(start) => match chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M:%S") {
            Ok(start) => Some(start.format("%H:%M").to_string()),
            Err(_) => None,
        },
        None => None,
    };
}

fn construct_event_url(json: &serde_json::Value) -> Option<String> {
    let scolaryear = match json["scolaryear"].as_str() {
        Some(scolaryear) => scolaryear.to_string(),
        None => return None,
    };

    let codemodule = match json["codemodule"].as_str() {
        Some(codemodule) => codemodule.to_string(),
        None => return None,
    };

    let codeinstance = match json["codeinstance"].as_str() {
        Some(codeinstance) => codeinstance.to_string(),
        None => return None,
    };

    let codeacti = match json["codeacti"].as_str() {
        Some(codeacti) => codeacti.to_string(),
        None => return None,
    };

    let codeevent = match json["codeevent"].as_str() {
        Some(codeevent) => codeevent.to_string(),
        None => return None,
    };

    Some(format!("/module/{}/{}/{}/{}/{}", scolaryear, codemodule, codeinstance, codeacti, codeevent))
}

pub fn list_events(autologin: &str) -> Result<Vec<Event>, Box<dyn error::Error>> {
    let today = chrono::Local::today();
    let today_str = today.format("%Y-%m-%d").to_string();
    let url = format!("{}/planning/load?format=json&start={}&end={}", autologin, today_str, today_str);

    let json = match intra::get_array_obj(&url) {
        Ok(json) => json,
        Err(e) => return match e {
            intra::Error::Empty => Ok(Vec::new()), // return empty JSON array
            _ => Err(e.into()), // return the error
        },
    };

    let mut list = Vec::new();

    for event in &json {
        // check if this event can have tokens
        match event["is_rdv"].as_str() {
            Some(is_rdv) => match is_rdv {
                "0" => (),
                _ => continue, // iterate over next event, skip this one
            },
            None => continue,
        };

        let code = match construct_event_url(&event) {
            Some(code) => code,
            None => return Err(Error::EventURL.into()),
        };

        let title = match event["acti_title"].as_str() {
            Some(title) => title.to_string(),
            None => return Err(Error::Title.into()),
        };

        let module = match event["titlemodule"].as_str() {
            Some(module) => module.to_string(),
            None => return Err(Error::Module.into()),
        };

        let date = today.naive_local();

        let start = match parse_time(&event, Time::Start) {
            Some(start) => start,
            None => return Err(Error::Time(Time::Start).into()),
        };
        let end = match parse_time(&event, Time::End) {
            Some(end) => end,
            None => return Err(Error::Time(Time::End).into()),
        };

        // fetch list of students registered to event
        let students = match fetch_students(autologin, &code) {
            Ok(students) => students,
            Err(e) => return Err(e.into()),
        };

        list.push(Event {
            code,
            title,
            module,
            date,
            start,
            end,
            students,
        });
    }

    Ok(list)
}
