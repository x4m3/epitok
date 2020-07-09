//! # Intranet authentication
//!
//! Authentication to the Epitech intranet, using account's autologin link.
//!
//! ## Example
//!
//! ```no_run
//! use epitok::auth::Auth;
//!
//! # fn main() -> Result<(), ()> {
//! let autologin = "https://intra.epitech.eu/auth-abcdefghijklmnopqrstuvwxyz1234567890abcd";
//! let mut user = Auth::new();
//! match user.sign_in(autologin) {
//!     Ok(()) => (),
//!     Err(e) => return Err(()),
//! }
//!
//!     println!("autologin : {}", user.autologin().as_ref().unwrap());
//!     println!("login     : {}", user.login().as_ref().unwrap());
//!     println!("name      : {}", user.name().as_ref().unwrap());
//! # Ok(())
//! # }
//! ```

use crate::intra;
use std::{error, fmt};

#[derive(Debug)]
/// Error possibilities
pub enum Error {
    /// Invalid autologin link: it may have been revoked
    Credentials,
    /// There is no email address associated with the account, should not be possible though
    NoLogin,
    /// There is no name associated with the account, should not be possible though
    NoName,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::Credentials => "Invalid autologin link provided",
            Error::NoLogin => "No login associated with intranet profile",
            Error::NoName => "No name associated with intranet profile",
        };
        write!(f, "{}", message)
    }
}

/// # Authentication
///
/// Authentication and identity verification to the intranet
///
/// You can use the library without this module, this is just an autologin storage and verifier
pub struct Auth {
    /// User's autologin link
    autologin: Option<String>,
    /// User's email address
    login: Option<String>,
    /// User's name
    name: Option<String>,
    /// Status
    status: bool,
}

impl Auth {
    /// Create with empty fields
    pub fn new() -> Auth {
        Auth {
            autologin: None,
            login: None,
            name: None,
            status: false,
        }
    }

    /// Sign-in with autologin link
    pub fn sign_in(&mut self, autologin: &str) -> Result<(), Box<dyn error::Error>> {
        // check autologin
        if !Auth::check_autologin(autologin) {
            return Err(Error::Credentials.into());
        }

        let url = format!("{}/user?format=json", autologin);

        let json = match intra::get_obj(&url) {
            Ok(intra_request) => intra_request,
            Err(e) => return Err(e.into()),
        };

        // get user's login
        let login = match json["login"].as_str() {
            Some(login) => login,
            None => return Err(Error::NoLogin.into()),
        };

        // get user's name
        let name = match json["title"].as_str() {
            Some(name) => name,
            None => return Err(Error::NoName.into()),
        };

        self.set_autologin(autologin);
        self.set_login(login);
        self.set_name(name);
        self.status = true;

        Ok(())
    }

    /// Sign-out
    pub fn sign_out(&mut self) {
        self.autologin = None;
        self.login = None;
        self.name = None;
        self.status = false;
    }

    /// Retrieve autologin link
    pub fn autologin(&self) -> &Option<String> {
        &self.autologin
    }

    fn set_autologin(&mut self, autologin: &str) {
        self.autologin = Some(autologin.to_string());
    }

    /// Retrieve email address
    pub fn login(&self) -> &Option<String> {
        &self.login
    }

    fn set_login(&mut self, login: &str) {
        self.login = Some(login.to_string());
    }

    /// Retrieve name
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    /// Get current status
    pub fn status(&self) -> &bool {
        &self.status
    }

    fn check_autologin(new: &str) -> bool {
        // prepare regex
        let rule = "^(https://intra.epitech.eu/auth-[a-z0-9]{40})$";
        let re = match regex::Regex::new(rule) {
            Ok(re) => re,
            Err(_) => return false,
        };

        // regex check
        re.is_match(new)
    }
}
