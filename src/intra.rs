//! # Intranet communication
//!
//! Communication to the Epitech intranet, to send and receive data

use std::collections::HashMap;
use std::{error, fmt};

#[derive(Debug)]
/// Error possibilities
pub enum Error {
    /// No network access
    Network,
    /// Account does not have permission to access resource
    AccessDenied,
    /// Can't access intranet (probably down)
    IntraDown,
    /// Failed to parse JSON reply
    Parsing,
    /// Empty JSON reply
    Empty,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::Network => "No internet access",
            Error::AccessDenied => "You do not have permission to access this resource",
            Error::IntraDown => "Could not connect to the epitech intranet",
            Error::Parsing => "Failed to parse retrieved data from the intranet",
            Error::Empty => "Empty JSON array",
        };
        write!(f, "{}", message)
    }
}

/// Make a request to get content from a URL
fn get_content(url: &str) -> Result<String, Error> {
    // make network request to intra
    let intra_req = match reqwest::blocking::get(url) {
        Ok(body) => body,
        Err(e) => {
            println!("{}", e);
            return Err(Error::Network);
        }
    };

    // user does not have access (bad autologin for example)
    if intra_req.status() == reqwest::StatusCode::FORBIDDEN {
        return Err(Error::AccessDenied);
    }

    // intra is probably down
    if intra_req.status() != reqwest::StatusCode::OK {
        return Err(Error::IntraDown);
    }

    // get request's content
    match intra_req.text() {
        Ok(raw) => Ok(raw),
        Err(e) => {
            println!("{}", e);
            Err(Error::Parsing)
        }
    }
}

/// Get JSON object from a URL
pub fn get_obj(url: &str) -> Result<serde_json::Value, Error> {
    let intra_request = match get_content(&url) {
        Ok(intra_request) => intra_request,
        Err(e) => return Err(e),
    };

    // parse json object
    match serde_json::from_str(&intra_request) {
        Ok(json) => Ok(json),
        Err(e) => {
            println!("{}", e);
            Err(Error::Parsing)
        }
    }
}

/// Get JSON array from a URL
pub fn get_array_obj(url: &str) -> Result<Vec<serde_json::Value>, Error> {
    let intra_request = match get_content(&url) {
        Ok(intra_request) => intra_request,
        Err(e) => return Err(e),
    };

    // parse json array of objects
    match serde_json::from_str(&intra_request) {
        Ok(json) => Ok(json),
        Err(e) => {
            println!("{}", e);
            Err(Error::Empty) // Return Error::empty if there is nothing in the object
        }
    }
}

/// Updates presence statuses of students for an event
///
/// # Arguments
///
/// * `autologin` - User autologin link
/// * `code_event` - Url code of the event
/// * `students` List of students and their presence status, made with `event.export_students`
pub fn update_presences(
    autologin: &str,
    event_code: &str,
    students: HashMap<String, String>,
) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}{}/updateregistered?format=json", autologin, event_code);

    let intra_req = match client.post(&url).form(&students).send() {
        Ok(req) => req,
        Err(e) => {
            println!("{}", e);
            return Err(Error::Network);
        }
    };

    // user does not have access (bad autologin for example)
    if intra_req.status() == reqwest::StatusCode::FORBIDDEN {
        return Err(Error::AccessDenied);
    }

    // intra is probably down or there is an unexpected error
    if intra_req.status() != reqwest::StatusCode::OK {
        return Err(Error::IntraDown);
    }

    Ok(())
}
