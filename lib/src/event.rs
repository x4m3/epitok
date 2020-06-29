use std::{error, fmt};
use crate::intra;
use crate::student::{Student, fetch_students, Presence};

#[derive(Debug)]
pub struct Event {
    code: String,
    title: String,
    module: String,
    date: chrono::NaiveDate,
    start: String,
    end: String,
    pub students: Vec<Student>,
}

impl Event {
    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_module(&self) -> &str {
        &self.module
    }

    pub fn get_date_str(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    pub fn get_time_start(&self) -> &str {
        &self.start
    }

    pub fn get_time_end(&self) -> &str {
        &self.end
    }

    fn set_student_presence(&mut self, login: &str, presence: Presence) -> bool {
        // find student with matching login
        let student = match self.students.iter_mut().find(|s| s.get_login() == login) {
            Some(student) => student,
            None => return false,
        };

        // update student presence
        student.set_presence(presence);
        true
    }

    pub fn set_student_present(&mut self, login: &str) -> bool {
        self.set_student_presence(login, Presence::Present)
    }

    pub fn set_student_missing(&mut self, login: &str) -> bool {
        self.set_student_presence(login, Presence::Missing)
    }

    fn set_all_students_presence(&mut self, presence: Presence) {
        let students = self.students.iter_mut();

        for student in students {
            student.set_presence(presence);
        }
    }

    pub fn set_all_students_present(&mut self) {
        self.set_all_students_presence(Presence::Present);
    }

    pub fn set_all_students_missing(&mut self) {
        self.set_all_students_presence(Presence::Missing);
    }

    pub fn set_remaining_students_missing(&mut self) {
        let students = self.students.iter_mut();

        for student in students {
            match student.get_presence() {
                Presence::None => student.set_presence(Presence::Missing),
                _ => (),
            }
        }
    }

    pub fn update_students(&self, autologin: &str) -> Result<(), Error> {
        // serialize to intra format
        // upload
        intra::update_presences(autologin, self.get_code());
        // check intra reply
        Ok(())
    }
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
    let scolaryear = json["scolaryear"].as_str()?;
    let codemodule = json["codemodule"].as_str()?;
    let codeinstance = json["codeinstance"].as_str()?;
    let codeacti = json["codeacti"].as_str()?;
    let codeevent = json["codeevent"].as_str()?;

    Some(format!("/module/{}/{}/{}/{}/{}", scolaryear, codemodule, codeinstance, codeacti, codeevent))
}

pub fn list_events(autologin: &str, raw_date: &str) -> Result<Vec<Event>, Box<dyn error::Error>> {
    let date = match chrono::NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(e) => return Err(e.into()),
    };
    let date_str = date.format("%Y-%m-%d").to_string();
    let url = format!("{}/planning/load?format=json&start={}&end={}", autologin, date_str, date_str);

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

        let date = date.clone();

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
