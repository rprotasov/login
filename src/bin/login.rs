extern crate one;

use self::one::*;

fn main() {
    let connection = establish_connection();
    
    println!("Login");
    println!("------------------\n");

    let user = match models::User::login(&connection) {
        Ok(user) => user,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    display_homepage(&user);
}

fn display_homepage(user: &models::User) {
    println!("\nWelcome back {} 👤\n", user.username);
    println!("Bio\n------------------");
    println!("{}\n", user.bio);
}

