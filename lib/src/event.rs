use std::error;
use crate::intra;

pub struct Event {
    title: String,
    module: String,
    code: String,
    date: chrono::Date<chrono::Local>,
    start: String,
    end: String,
    // TODO: student type
}

impl Event {
    pub fn new() {}
    pub fn fetch_students() {}
}

enum Time {
    Start,
    End
}

fn parse_time(json: &serde_json::Value, time: Time) -> String {
    let time = match time {
        Time::Start => "start",
        Time::End => "end",
    };

    return match json[time].as_str() {
        Some(start) => match chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M:%S") {
            Ok(start) => start.format("%H:%M").to_string(),
            Err(_) => format!("No {} time", time),
        },
        None => format!("No {} time", time),
    };
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
        let title = match event["acti_title"].as_str() {
            Some(title) => title.to_string(),
            None => "Unknown Title".to_string(),
        };

        let module = match event["titlemodule"].as_str() {
            Some(module) => module.to_string(),
            None => "Unknown Module".to_string(),
        };

        // TODO: code

        let date = today.clone();

        let start = parse_time(&event, Time::Start);
        let end = parse_time(&event, Time::End);

        list.push(Event {
            title,
            module,
            code,
            date,
            start,
            end,
        });
    }

    // for each event that requires a token, add it and fetch list of students

    Ok(list)
}
