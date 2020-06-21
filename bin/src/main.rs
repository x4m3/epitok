use epitok_lib::auth::Auth;

fn print(user: &Auth) {
    println!("login     : {:?}", user.get_login());
    println!("autologin : {:?}", user.get_autologin());
    println!();
}

fn main() {
    let user = match Auth::new("https://intra.epitech.eu/auth-") {
        Ok(user) => user,
        Err(e) => {
            println!("could not login: {}", e);
            return;
        }
    };
    print(&user);
}