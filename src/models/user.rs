use diesel::prelude::*;

#[derive(AsChangeset, Debug, Identifiable, Queryable, Selectable)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = crate::schema::user_profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfile {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::user_profile)]
pub struct NewUserProfile {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
