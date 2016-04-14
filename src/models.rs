extern crate diesel;
extern crate rand;

use super::schema::users;
use super::error::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::io::{self, stdin, Write};
use crypto::bcrypt;
use rand::Rng;

type UserInformation = (String, String, String);

const ENCRYPTION_ROUNDS: u32 = 5;
const DIGEST_LENGTH: usize = 24;
const MAX_SALT_LENGTH: usize = 16;

#[derive(Queryable, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub bio: String,
    pub password_digest: Vec<u8>,
    pub salt: Vec<u8>,
}

#[insertable_into(users)]
pub struct NewUser {
    pub username: String,
    pub bio: String,
    pub password_digest: Vec<u8>,
    pub salt: Vec<u8>,
}

impl User {
    pub fn create(conn: &PgConnection) -> Result<NewUser, DbError> {
        let (username, password, bio) = match User::gather_new_user_information(conn) {
            Ok(info) => info,
            Err(e) => return Err(e),
        };

        let salt = User::make_salt();
        let password_digest = User::hash_password(&password, &salt);
        
        let user = NewUser {
            username: username,
            bio: bio,
            password_digest: password_digest,
            salt: salt,
        };

        Ok(user)
    }

    fn gather_new_user_information(conn: &PgConnection) -> Result<UserInformation, DbError> {
        let username = get_input_with("Username: ");
        
        if User::is_existing_username(&username, &conn) {
            return Err(
                DbError::ExistingUsername(username)
            );
        }

        let password = get_input_with("Password: ");

        let mut bio = get_input_with("And a short description of yourself [leave blank to do later]: ");

        // Default bio if none is given
        if bio.len() == 0 {
            bio = "It's looking a bit empty here".to_string();
        }

        Ok(
            (username, password, bio)
        )
    }

    fn is_existing_username(name: &String, conn: &PgConnection) -> bool {
        use schema::users::dsl::*;

        let count = 
            users.filter(username.eq(name)).load::<User>(conn)
                .expect("Error loading users");

        count.len() != 0
    }


    fn make_salt() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut salt = Vec::with_capacity(MAX_SALT_LENGTH);

        while salt.len() != MAX_SALT_LENGTH {
            if rng.gen() {
                salt.push(rng.gen::<u8>());
            }
        }
    
        salt
    }

    fn hash_password(password: &String, salt: &Vec<u8>) -> Vec<u8> { 
        let mut output = [0u8; DIGEST_LENGTH];

        bcrypt::bcrypt(ENCRYPTION_ROUNDS, &salt[..], &password.as_bytes(), &mut output);
    
        let mut digest: Vec<u8> = Vec::with_capacity(DIGEST_LENGTH);

        for i in 0..DIGEST_LENGTH {
            digest.push(output[i]);
        }
    
        digest 
    }

    pub fn login(conn: &PgConnection) -> Result<User, DbError> {
        let username = get_input_with("Username: ");
        let password = get_input_with("Password: ");

        User::get_user_by_login(&username, &password, conn)
    }

    fn get_user_by_login(name: &String, password: &String, conn: &PgConnection) -> Result<User, DbError> {
        use schema::users::dsl::*;

        let results = 
            users.filter(username.eq(name)).load::<User>(conn)
                .expect("Error loading users");
    
        if results.len() != 1 {
            return Err(
                DbError::IncorrectUsernamePasswordCombination
            );
        }

        let user: User = results[0].clone();

        let digest: Vec<u8> = User::hash_password(password, &user.salt);
    
        if digest != user.password_digest {
            return Err(
                DbError::IncorrectUsernamePasswordCombination
            );
        }

        Ok(user)
    }
}

impl NewUser {
    pub fn save(&self, conn: &PgConnection) -> User {
        use schema::users;

        let saved_user = 
            diesel::insert(self).into(users::table).get_result(conn)
                .expect("Error registering user");

        saved_user
    }
}


fn get_input_with(output: &'static str) -> String {
    print(output);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input[..(input.len() - 1)].to_string() // Drops the new line character
}

fn print(output: &'static str) {
    print!("{}", output);
    io::stdout().flush().unwrap();
}
