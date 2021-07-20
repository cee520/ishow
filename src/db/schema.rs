table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        mobile -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        given_name -> Varchar,
        encrypted_password -> Varchar,
        avatar -> Varchar,
        locked_at -> Timestamp,
        current_sign_in_at -> Timestamp,
        current_sign_in_ip -> Varchar,
        last_sign_in_at -> Timestamp,
        last_sign_in_ip -> Varchar,
        sign_in_count -> Int4,
    }
}
