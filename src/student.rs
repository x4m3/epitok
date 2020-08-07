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
    /// Student has an unknown or invalid presence status
    InvalidPresence,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::Login => "Student does not have an epitech login",
            Error::Name => "Student does not have a name",
            Error::InvalidPresence => "Student has a invalid presence code",
        };
        write!(f, "{}", message)
    }
}

/// Get list of students from an event
pub async fn fetch_students(
    autologin: &str,
    event_code: &str,
) -> Result<Vec<Student>, Box<dyn error::Error>> {
    let url = format!("{}{}/registered?format=json", autologin, event_code);

    let json = match intra::get_array_obj(&url).await {
        Ok(json) => json,
        Err(e) => {
            return match e {
                intra::Error::Empty => Ok(Vec::new()), // return empty JSON array
                _ => Err(e.into()),                    // return the error
            };
        }
    };

    let mut list = Vec::new();

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
            Some(presence) => match presence {
                "present" => Presence::Present,
                "absent" => Presence::Missing,
                "N/A" => Presence::NotApplicable,
                "failed" => Presence::Failed,
                _ => return Err(Error::InvalidPresence.into()),
            },
            None => Presence::None,
        };

        list.push(Student {
            login,
            name,
            presence,
        });
    }

    Ok(list)
}
