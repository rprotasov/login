extern crate one;

use self::one::*;

fn main() {
    let connection = establish_connection();
    
    println!("Register");
    println!("------------------\n");
    
    let registered_user = match models::User::create(&connection) {
        Ok(user) => user.save(&connection),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    
    println!("\nThank you for registering {}!", registered_user.username);
}
