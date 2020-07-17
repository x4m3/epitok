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
    /// Intra error
    IntraError(intra::Error),
    /// Not signed in
    NotSignedIn,
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
        let message: String = match *self {
            Error::IntraError(intra::Error::Network) => intra::Error::Network.to_string(),
            Error::IntraError(intra::Error::AccessDenied) => intra::Error::AccessDenied.to_string(),
            Error::IntraError(intra::Error::NotFound) => intra::Error::NotFound.to_string(),
            Error::IntraError(intra::Error::IntraDown) => intra::Error::IntraDown.to_string(),
            Error::IntraError(intra::Error::Parsing) => intra::Error::Parsing.to_string(),
            Error::IntraError(intra::Error::Empty) => intra::Error::Empty.to_string(),
            Error::NotSignedIn => "You are not signed in".into(),
            Error::Credentials => "Invalid autologin link provided".into(),
            Error::NoLogin => "No login associated with intranet profile".into(),
            Error::NoName => "No name associated with intranet profile".into(),
        };
        write!(f, "{}", message)
    }
}

/// Authentication status
pub enum Status {
    /// Signed in
    SignedIn,
    /// Signed out
    SignedOut,
    /// Could not sign in
    Error(Error),
}

impl Default for Status {
    fn default() -> Self {
        Status::SignedOut
    }
}

/// # Authentication
///
/// Authentication and identity verification to the intranet
///
/// You can use the library without this module, this is just an autologin storage and verifier
#[derive(Default)]
pub struct Auth {
    /// User's autologin link
    autologin: Option<String>,
    /// User's email address
    login: Option<String>,
    /// User's name
    name: Option<String>,
    /// Status
    status: Status,
}

impl Auth {
    /// Create with empty fields
    pub fn new() -> Self {
        Default::default()
    }

    /// Sign-in with autologin link
    pub fn sign_in(&mut self, autologin: &str) -> Result<(), Box<dyn error::Error>> {
        // check autologin
        if !Self::check_autologin(autologin) {
            self.status = Status::Error(Error::Credentials);
            return Err(Error::Credentials.into());
        }

        let url = format!("{}/user?format=json", autologin);

        let json = match intra::get_obj(&url) {
            Ok(intra_request) => intra_request,
            Err(e) => {
                self.status = Status::Error(Error::IntraError(e));
                return Err(e.into());
            }
        };

        // get user's login
        let login = match json["login"].as_str() {
            Some(login) => login,
            None => {
                self.status = Status::Error(Error::NoLogin);
                return Err(Error::NoLogin.into());
            }
        };

        // get user's name
        let name = match json["title"].as_str() {
            Some(name) => name,
            None => {
                self.status = Status::Error(Error::NoName);
                return Err(Error::NoName.into());
            }
        };

        self.set_autologin(autologin);
        self.set_login(login);
        self.set_name(name);
        self.status = Status::SignedIn;

        Ok(())
    }

    /// Sign-out
    pub fn sign_out(&mut self) {
        self.autologin = None;
        self.login = None;
        self.name = None;
        self.status = Status::SignedOut;
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
    pub fn status(&self) -> &Status {
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
