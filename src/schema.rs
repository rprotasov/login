table! {
    users {
        id -> Integer,
        username -> VarChar,
        bio -> Text,
        password_digest -> Binary,
        salt -> Binary,
    }
}
