use epitok_lib::auth::Auth;
use epitok_lib::event;

fn print(user: &Auth) {
    println!("login     : {}", user.get_login());
    println!("autologin : {}", user.get_autologin());
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

    let mut today_events = match event::list_events(user.get_autologin(), "2020-06-16") {
        Ok(events) => events,
        Err(e) => {
            println!("could not get events: {}", e);
            return;
        }
    };

    // let event = &mut today_events[0]; // get first event

    for event in &mut today_events {
        println!("{} --- {}", event.get_title(), event.get_module());

        for (i, student) in event.students.iter().enumerate() {
            println!("items[{}][login]={}", i, student.get_login());
            println!("items[{}][present]={:?}", i, student.get_presence());
        }

        match event.set_student_present("first.last@epitech.eu") {
            true => (),
            false => eprintln!("could not set user present\n"),
        }
    }
}