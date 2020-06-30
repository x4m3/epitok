use crate::intra;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Credentials,
    NoLogin,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::Credentials => "Invalid autologin link",
            Error::NoLogin => "You do not have a login associated with your intranet profile",
        };
        write!(f, "{}", message)
    }
}

pub struct Auth {
    autologin: String,
    login: String,
}

impl Auth {
    pub fn new(autologin: &str) -> Result<Auth, Box<dyn error::Error>> {
        // check autologin
        if !Auth::check_autologin(autologin) {
            return Err(Error::Credentials.into());
        }

        // sign in
        let login = match Auth::sign_in(autologin) {
            Ok(login) => login,
            Err(e) => return Err(e),
        };

        let user = Auth {
            autologin: autologin.to_string(),
            login,
        };

        Ok(user)
    }

    pub fn get_autologin(&self) -> &str {
        &self.autologin
    }

    pub fn get_login(&self) -> &str {
        &self.login
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

    fn sign_in(autologin: &str) -> Result<String, Box<dyn error::Error>> {
        let url = format!("{}/user?format=json", autologin);

        let json = match intra::get_obj(&url) {
            Ok(intra_request) => intra_request,
            Err(e) => return Err(e.into()),
        };

        // get user's login
        match json["login"].as_str() {
            Some(login) => Ok(login.to_string()),
            None => Err(Error::NoLogin.into()),
        }
    }
}
