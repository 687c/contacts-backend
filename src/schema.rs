// @generated automatically by Diesel CLI.

diesel::table! {
    contact (contact_id) {
        contact_id -> Int4,
        user_id -> Nullable<Int4>,
        email -> Varchar,
        #[max_length = 30]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_profile (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(contact -> user_profile (user_id));

diesel::allow_tables_to_appear_in_same_query!(contact, user_profile,);
