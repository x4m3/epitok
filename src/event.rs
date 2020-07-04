//! # Event management
//!
//! Event handling
//!
//! Here you can get events, registered students with their statuses, you can modify and upload them back to the intra.
//!
//! ## Example
//!
//! ```no_run
//! # use std::error::Error;
//! use epitok::event;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let autologin = "https://intra.epitech.eu/auth-autologin";
//!
//! // Get list of events
//! let mut events = event::list_events(autologin, "2020-07-04")?;
//!
//! // Select the first event
//! let first_event = &mut events[0];
//!
//! // Print information about event
//! println!("code: {}", first_event.code());
//! println!("title: {}", first_event.title());
//! println!("module: {}", first_event.module());
//!
//! // Reset status of all students
//! first_event.set_all_students_none();
//!
//! // Modify presence status to some students
//! first_event.set_student_present("first.last@epitech.eu");
//! first_event.set_student_missing("anony.mous@epitech.eu");
//! first_event.set_student_not_applicable("a.b@epitech.eu");
//!
//! // Upload changes to the intra
//! first_event.save_changes(autologin)?;
//!
//! // Display new presence statuses
//! for student in first_event.students().iter() {
//!     println!("{} - {}", student.get_login(), student.get_presence());
//! }
//!
//! # Ok(())
//! # }
//! ```

use crate::intra;
use crate::student::{fetch_students, Presence, Student};
use std::collections::HashMap;
use std::{error, fmt};

#[derive(Debug)]
/// # Event
///
/// Information about an event
pub struct Event {
    /// URL code of the event (aka the event ID in the intra)
    code: String,
    /// Name of the event
    title: String,
    /// Module of the event (for clarity)
    module: String,
    /// When event starts
    start: String,
    /// When event ends
    end: String,
    /// Registered students
    students: Vec<Student>,
}

impl Event {
    /// Get URL code
    ///
    /// # Output format
    ///
    /// `/module/2019/X-XXX-000/XXX-0-0/acti-000000/event-000000`
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get name
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get module name
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Get start time in `HH:MM` format
    pub fn start(&self) -> &str {
        &self.start
    }

    /// Get finish time in `HH:MM` format
    pub fn end(&self) -> &str {
        &self.end
    }

    /// Get list of students
    pub fn students(&self) -> &Vec<Student> {
        &self.students
    }

    /// Set presence type of a student
    ///
    /// # Arguments
    ///
    /// * `login` - Student email address
    /// * `presence` - Type of presence to set
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

    /// Set student present
    ///
    /// # Arguments
    ///
    /// * `login` - Student email address
    pub fn set_student_present(&mut self, login: &str) -> bool {
        self.set_student_presence(login, Presence::Present)
    }

    /// Set student missing
    ///
    /// # Arguments
    ///
    /// * `login` - Student email address
    pub fn set_student_missing(&mut self, login: &str) -> bool {
        self.set_student_presence(login, Presence::Missing)
    }

    /// Remove student presence
    ///
    /// # Arguments
    ///
    /// * `login` - Student email address
    pub fn set_student_none(&mut self, login: &str) -> bool {
        self.set_student_presence(login, Presence::None)
    }

    /// Set student N/A
    ///
    /// # Arguments
    ///
    /// * `login` - Student email address
    pub fn set_student_not_applicable(&mut self, login: &str) -> bool {
        self.set_student_presence(login, Presence::NotApplicable)
    }

    fn set_all_students_presence(&mut self, presence: Presence) {
        let students = self.students.iter_mut();

        for student in students {
            student.set_presence(presence);
        }
    }

    /// Set all students as present
    pub fn set_all_students_present(&mut self) {
        self.set_all_students_presence(Presence::Present);
    }

    /// Set all students as missing
    pub fn set_all_students_missing(&mut self) {
        self.set_all_students_presence(Presence::Missing);
    }

    /// Remove presences for all students
    pub fn set_all_students_none(&mut self) {
        self.set_all_students_presence(Presence::None);
    }

    /// Set presences for all students as Not applicable
    pub fn set_all_students_not_applicable(&mut self) {
        self.set_all_students_presence(Presence::NotApplicable);
    }

    fn set_remaining_students_presence(&mut self, presence: Presence) {
        let students = self.students.iter_mut();

        for student in students {
            if let Presence::None = student.get_presence() {
                student.set_presence(presence)
            }
        }
    }

    /// Set students who do not have a presence status as present
    pub fn set_remaining_students_present(&mut self) {
        self.set_remaining_students_presence(Presence::Present)
    }

    /// Set students who do not have a presence status as missing
    pub fn set_remaining_students_missing(&mut self) {
        self.set_remaining_students_presence(Presence::Missing)
    }

    /// Export registered students to intra format (to be uploaded)
    ///
    /// The intra API uses `url-encoded` forms as a format to upload students and their statuses:
    /// - `items[x][login]=first.last@epitech.eu`
    /// - `items[x][present]=presence`
    ///
    /// where
    /// - `x` is the position of the student in the array
    /// - `first.last@epitech.eu` is the email of the student
    /// - `presence` is the presence status of the student (see `student::Presence` for more information)
    fn export_students(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();

        for (i, student) in self.students.iter().enumerate() {
            // student login
            let login_k = format!("items[{}][login]", i);
            let login_v = student.get_login().to_string();
            hm.insert(login_k, login_v);

            // student presence value
            let presence_k = format!("items[{}][present]", i);
            let presence_v = student.get_presence().to_string();
            hm.insert(presence_k, presence_v);
        }
        hm
    }

    /// Save changes to the intra (upload them)
    ///
    /// # Arguments
    ///
    /// * `autologin` - Autologin link. If you use the `epitok::auth::Auth` struct, use its `get_autologin` method
    ///
    /// # Note
    /// Before saving changes, setting every student without a status as missing
    pub fn save_changes(&mut self, autologin: &str) -> Result<(), Box<dyn error::Error>> {
        // make sure every student has a valid status
        self.set_remaining_students_missing();

        // export students to intra format
        let students = self.export_students();

        // upload and check intra reply
        intra::update_presences(autologin, self.code(), students)?;

        Ok(())
    }
}

#[derive(Debug)]
/// Error possibilities
pub enum Error {
    /// Event does not have a URL
    EventURL,
    /// Event does not have a title
    Title,
    /// Event is not linked to any modules
    Module,
    /// Event does not have a starting time
    TimeStart,
    /// Event does not have a finish time
    TimeEnd,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::EventURL => "Event doesn't have a url (how is this even possible?)",
            Error::Title => "This event does not have a title",
            Error::Module => "This event does not belong to a module",
            Error::TimeStart => "This event does not have a starting time",
            Error::TimeEnd => "This event does not have a finish time",
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
/// Time representation
enum Time {
    /// Event start
    Start,
    /// Event end
    End,
}

/// Parse start or end time from JSON
fn parse_time(json: &serde_json::Value, time: Time) -> Option<String> {
    let time = match time {
        Time::Start => "start",
        Time::End => "end",
    };

    match json[time].as_str() {
        Some(start) => match chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%d %H:%M:%S") {
            Ok(start) => Some(start.format("%H:%M").to_string()),
            Err(_) => None,
        },
        None => None,
    }
}

/// Construct the URL for the event based on the intra information
fn construct_event_url(json: &serde_json::Value) -> Option<String> {
    let scolaryear = json["scolaryear"].as_str()?;
    let codemodule = json["codemodule"].as_str()?;
    let codeinstance = json["codeinstance"].as_str()?;
    let codeacti = json["codeacti"].as_str()?;
    let codeevent = json["codeevent"].as_str()?;

    Some(format!(
        "/module/{}/{}/{}/{}/{}",
        scolaryear, codemodule, codeinstance, codeacti, codeevent
    ))
}

/// Show events of a particular date
///
/// # Arguments
///
/// * `autologin` - User autologin link
/// * `raw_date` - Date in `YYYY-MM-DD` format
///
/// # Example
///
/// Get a vector of events from a particular date and print their name
///
/// ```no_run
/// use epitok::event::list_events;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let date_str = "2020-07-01";
/// let autologin = "https://intra.epitech.eu/auth-abcdefghijklmnopqrstuvwxyz1234567890abcd";
///
/// let events = list_events(autologin, date_str)?;
/// for event in events {
///     println!("event: {} - {}", event.title(), event.module());
/// }
/// # Ok(())
/// # }
/// ```
pub fn list_events(autologin: &str, raw_date: &str) -> Result<Vec<Event>, Box<dyn error::Error>> {
    // check if the date provided is valid
    if let Err(e) = chrono::NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d") {
        return Err(e.into());
    }

    let url = format!(
        "{}/planning/load?format=json&start={}&end={}",
        autologin, raw_date, raw_date
    );

    let json = match intra::get_array_obj(&url) {
        Ok(json) => json,
        Err(e) => {
            return match e {
                intra::Error::Empty => Ok(Vec::new()), // return empty JSON array
                _ => Err(e.into()),                    // return the error
            };
        }
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

        let start = match parse_time(&event, Time::Start) {
            Some(start) => start,
            None => return Err(Error::TimeStart.into()),
        };
        let end = match parse_time(&event, Time::End) {
            Some(end) => end,
            None => return Err(Error::TimeEnd.into()),
        };

        // fetch list of students registered to event
        let students = match fetch_students(autologin, &code) {
            Ok(students) => students,
            Err(e) => return Err(e),
        };

        list.push(Event {
            code,
            title,
            module,
            start,
            end,
            students,
        });
    }

    Ok(list)
}
