#[derive(Debug)]
pub enum AuthStatus {
    Valid,
    InvalidCredentials,
    NetworkError,
    AccessDenied,
    IntraDown,
    UnknownError,
}

pub struct Authentication {
    autologin: Option<String>,
    login: Option<String>,
    status: Option<AuthStatus>,
}

impl Authentication {
    pub fn new() -> Authentication {
        Authentication {
            autologin: None,
            login: None,
            status: None,
        }
    }

    pub fn get_autologin(&self) -> &Option<String> {
        &self.autologin
    }

    pub fn set_autologin(&mut self, new: &str) -> bool {

        // prepare regex
        let rule = "^(https://intra.epitech.eu/auth-[a-z0-9]{40})$";
        let re = match regex::Regex::new(rule) {
            Ok(re) => re,
            Err(_) => return false,
        };

        // regex check
        if re.is_match(new) == false {
            return false;
        }

        self.autologin = Some(new.to_string());

        true
    }

    pub fn get_login(&self) -> &Option<String> {
        &self.login
    }

    pub fn get_status(&self) -> &Option<AuthStatus> {
        &self.status
    }

    pub fn sign_in(&mut self) {
        // make sure credentials are valid
        let url = match self.get_autologin() {
            Some(autologin) => format!("{}/user?format=json", autologin),
            None => {
                self.status = Some(AuthStatus::InvalidCredentials);
                return;
            }
        };

        // make network request to intra
        let intra_req = match reqwest::blocking::get(&url) {
            Ok(body) => body,
            Err(e) => {
                println!("{}", e);
                self.status = Some(AuthStatus::NetworkError);
                return;
            }
        };

        // user does not have access (bad autologin for example)
        if intra_req.status() == reqwest::StatusCode::FORBIDDEN {
            self.status = Some(AuthStatus::AccessDenied);
            return;
        }

        // intra is probably down
        if intra_req.status() != reqwest::StatusCode::OK {
            self.status = Some(AuthStatus::IntraDown);
            return;
        }

        // get request's content
        let raw = match intra_req.text() {
            Ok(raw) => raw,
            Err(_) => {
                self.status = Some(AuthStatus::UnknownError);
                return;
            }
        };

        // parse json
        let json: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(json) => json,
            Err(_) => {
                self.status = Some(AuthStatus::UnknownError);
                return;
            }
        };

        // store login
        match json["login"].as_str() {
            Some(login) => self.login = Some(login.to_string()),
            None => {
                self.status = Some(AuthStatus::UnknownError);
                return;
            }
        }

        self.status = Some(AuthStatus::Valid);
    }
}