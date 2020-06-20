mod authentication;

use authentication::Authentication;

fn print(user: &Authentication) {
    println!("login     : {:?}", user.get_login());
    println!("autologin : {:?}", user.get_autologin());
    println!("status    : {:?}", user.get_status());
    println!();
}

fn main() {
    let mut phil = Authentication::new();

    print(&phil);

    phil.set_autologin("https://intra.epitech.eu/auth-");
    phil.sign_in();

    print(&phil);

}