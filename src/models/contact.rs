use diesel::prelude::*;

use crate::models::user::UserProfile;

#[derive(AsChangeset, Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(UserProfile, foreign_key = user_id))]
#[diesel(primary_key(contact_id))]
#[diesel(table_name = crate::schema::contact)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contact {
    contact_id: i32,
    user_id: Option<i32>,
    email: String,
    phone: Option<String>,
    city: Option<String>,
    country: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::contact)]
pub struct NewContact {
    user_id: Option<i32>,
    email: String,
    phone: Option<String>,
    city: Option<String>,
    country: Option<String>,
}
