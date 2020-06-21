use std::fmt;

pub enum Error {
    Credentials,
    Network,
    AccessDenied,
    IntraDown,
    Parsing,
    NoLogin,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::Credentials => "Invalid autologin link",
            Error::Network => "No internet access",
            Error::AccessDenied => "You do not have permission to access this resource",
            Error::IntraDown => "Could not connect to the epitech intranet",
            Error::Parsing => "Failed to parse retrieved data from the intranet",
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
    pub fn new(autologin: &str) -> Result<Auth, Error> {
        // check autologin
        if Auth::check_autologin(autologin) == false {
            return Err(Error::Credentials);
        }

        // sign in
        let login = match Auth::sign_in(autologin) {
            Ok(login) => login,
            Err(e) => return Err(e),
        };

        let user = Auth {
            autologin: autologin.to_string(),
            login: login.to_string(),
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

    fn sign_in(autologin: &str) -> Result<String, Error> {
        let url = format!("{}/user?format=json", autologin);

        // make network request to intra
        let intra_req = match reqwest::blocking::get(&url) {
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
        let raw = match intra_req.text() {
            Ok(raw) => raw,
            Err(e) => {
                println!("{}", e);
                return Err(Error::Parsing);
            }
        };

        // parse json
        let json: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(json) => json,
            Err(e) => {
                println!("{}", e);
                return Err(Error::Parsing);
            }
        };

        // get user's login
        match json["login"].as_str() {
            Some(login) => Ok(login.to_string()),
            None => Err(Error::NoLogin),
        }
    }

}
