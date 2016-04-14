use std::fmt;

#[derive(Debug)]
pub enum DbError {
    ExistingUsername(String),
    IncorrectUsernamePasswordCombination,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::ExistingUsername(ref name) => write!(f, "user `{}` exits", name.clone()),
            DbError::IncorrectUsernamePasswordCombination => {
                write!(f, "incorrect username and password combination")
            }
        }
    }
}
