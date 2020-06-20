mod authentication;

use authentication::Authentication;

fn print(user: &Authentication) {
    println!("login     : {:?}", user.get_autologin());
    println!("autologin : {:?}", user.get_login());
    println!("status    : {:?}", user.get_status());
    println!();
}

fn main() {
    let mut phil = Authentication::new();

    print(&phil);

    let ret = phil.sign_in("autologin-here");
    println!("ret: {}", ret);

    print(&phil);

}