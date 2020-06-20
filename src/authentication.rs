const INTRA_URL: &str = "https://intra.epitech.eu/auth-";

#[derive(Debug)]
pub enum AuthStatus {
    NotSignedIn,
    SignedIn,
    NetworkError,
    BadCredentials,
}

pub struct Authentication {
    autologin: Option<String>,
    login: Option<String>,
    status: AuthStatus,
}

impl Authentication {
    pub fn new() -> Authentication {
        Authentication {
            autologin: None,
            login: None,
            status: AuthStatus::NotSignedIn,
        }
    }

    pub fn get_autologin(&self) -> &Option<String> {
        &self.autologin
    }

    pub fn get_login(&self) -> &Option<String> {
        &self.login
    }

    pub fn get_status(&self) -> &AuthStatus {
        &self.status
    }

    pub fn sign_in(&mut self, autologin: &str) -> bool {
        // set new autologin
        self.autologin = Some(autologin.to_string());

        // make network request to intra
        let url = match self.get_autologin() {
            Some(autologin) => format!("{}{}/user?format=json", INTRA_URL, autologin),
            None => return false,
        };

        let body = match reqwest::blocking::get(&url) {
            Ok(body) => body,
            Err(e) => return false,
        };

        // set status
        // set login if status is ok
        true
    }
}