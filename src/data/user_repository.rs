use crate::models::user::{NewUserProfile, UserProfile};
use crate::schema::user_profile;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_user(conn: &mut PgConnection, new_user: NewUserProfile) -> UserProfile {
    diesel::insert_into(user_profile::table)
        .values(&new_user)
        .returning(UserProfile::as_returning())
        .get_result(conn)
        .expect("error inserting new record into database")
}

pub fn update_user_profile_by_id(
    conn: &mut PgConnection,
    updated_user_profile: UserProfile,
) -> UserProfile {
    diesel::update(user_profile::table)
        .set(updated_user_profile)
        .get_result(conn)
        .expect("error updating specified recor")
}

pub fn delete_user_profile_by_id(user_id: &i32, conn: &mut PgConnection) {
    use crate::schema::user_profile::dsl::*;

    diesel::delete(user_profile.filter(user_id.eq(user_id)))
        .execute(conn)
        .expect("error deleting the user record");
}

