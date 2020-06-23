use std::{error, fmt};
use crate::intra;

#[derive(Debug)]
enum Presence {
    // there is no status yet
    None,
    // "present"
    Present,
    // "absent"
    Missing,
    // "N/A"
    NotApplicable,
    // "failed"
    Failed,
}

#[derive(Debug)]
pub struct Student {
    login: String,
    name: String,
    presence: Presence,
}

#[derive(Debug)]
pub enum Error {
    Login,
    Name,
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

pub fn fetch_students(autologin: &str, event: &str) -> Result<Vec<Student>, Box<dyn error::Error>> {
    let url = format!("{}{}/registered?format=json", autologin, event);

    let json = match intra::get_array_obj(&url) {
        Ok(json) => json,
        Err(e) => return match e {
            intra::Error::Empty => Ok(Vec::new()), // return empty JSON array
            _ => Err(e.into()), // return the error
        },
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