use std::collections::HashMap;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Network,
    AccessDenied,
    IntraDown,
    Parsing,
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

fn request(url: &str) -> Result<String, Error> {
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
    return match intra_req.text() {
        Ok(raw) => Ok(raw),
        Err(e) => {
            println!("{}", e);
            Err(Error::Parsing)
        }
    };
}

pub fn get_obj(url: &str) -> Result<serde_json::Value, Error> {
    let intra_request = match request(&url) {
        Ok(intra_request) => intra_request,
        Err(e) => return Err(e),
    };

    // parse json object
    return match serde_json::from_str(&intra_request) {
        Ok(json) => Ok(json),
        Err(e) => {
            println!("{}", e);
            Err(Error::Parsing)
        }
    };
}

pub fn get_array_obj(url: &str) -> Result<Vec<serde_json::Value>, Error> {
    let intra_request = match request(&url) {
        Ok(intra_request) => intra_request,
        Err(e) => return Err(e),
    };

    // parse json array of objects
    return match serde_json::from_str(&intra_request) {
        Ok(json) => Ok(json),
        Err(e) => {
            println!("{}", e);
            Err(Error::Empty) // Return Error::empty if there is nothing in the object
        }
    };
}

pub fn update_presences(
    autologin: &str,
    code_event: &str,
    students: HashMap<String, String>,
) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}{}/updateregistered?format=json", autologin, code_event);

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
