//! # Student
//!
//! Student handling
//!
//! As a user of this library you should not need to use the contents of this module,
//! they are used in internal modules of the library.

use crate::intra;
use std::{error, fmt};

#[derive(Debug, Clone, Copy)]
/// # Presence
///
/// Presence options for students
pub enum Presence {
    /// Student does not have a status yet
    None,
    /// Student was here
    Present,
    /// Student was not here
    Missing,
    /// Student can't be here
    NotApplicable,
    /// Student tried to enter a token but failed to save it
    ///
    /// # Note
    /// This value is not used in this library but it can be possible
    Failed,
}

impl Presence {
    pub fn from(s: &str) -> Self {
        match s {
            "present" => Presence::Present,
            "Present" => Presence::Present,
            "absent" => Presence::Missing,
            "Missing" => Presence::Missing,
            "N/A" => Presence::NotApplicable,
            "NotApplicable" => Presence::NotApplicable,
            "failed" => Presence::Failed,
            "Failed" => Presence::Failed,
            "None" => Presence::None,
            _ => Presence::Failed,
        }
    }
}

impl fmt::Display for Presence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Presence::None => "", // on the intra the json value is `null`
            Presence::Present => "present",
            Presence::Missing => "absent",
            Presence::NotApplicable => "N/A",
            Presence::Failed => "failed",
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
/// # Student
///
/// Information about a student in an event
pub struct Student {
    /// Email address
    login: String,
    /// Student name
    name: String,
    /// Student presence status
    presence: Presence,
}

impl Student {
    /// Get student's email address
    pub fn get_login(&self) -> &str {
        &self.login
    }

    /// Get student's name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get student's presence
    pub fn get_presence(&self) -> &Presence {
        &self.presence
    }

    /// Set student's presence
    pub fn set_presence(&mut self, presence: Presence) {
        self.presence = presence
    }
}

#[derive(Debug)]
/// Error possibilities
pub enum Error {
    /// Student does not have an email address
    Login,
    /// Student does not have a name
    Name,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::Login => "Student does not have an epitech login",
            Error::Name => "Student does not have a name",
        };
        write!(f, "{}", message)
    }
}

/// Get list of students from an event
pub async fn fetch_students(
    list: &mut Vec<Student>,
    autologin: &str,
    event_code: &str,
) -> Result<usize, Box<dyn error::Error>> {
    let url = format!("{}{}/registered?format=json", autologin, event_code);

    let json = match intra::get_array_obj(&url).await {
        Ok(json) => json,
        Err(e) => {
            return match e {
                intra::Error::Empty => Ok(0), // No students have signed up for this event
                _ => Err(e.into()),           // Return the intra error
            };
        }
    };

    // clear vector if it's not empty
    if !list.is_empty() {
        list.clear();
    }

    let mut number_students = 0;

    for student in &json {
        let login = match student["login"].as_str() {
            Some(login) => login.to_string(),
            None => return Err(Error::Login.into()),
        };

        let name = match student["title"].as_str() {
            Some(name) => name.to_string(),
            None => return Err(Error::Name.into()),
        };

        let presence = match student["present"].as_str() {
            Some(presence) => Presence::from(presence),
            None => Presence::None,
        };

        list.push(Student {
            login,
            name,
            presence,
        });

        number_students += 1;
    }

    Ok(number_students)
}
