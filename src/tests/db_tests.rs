#[cfg(test)]
mod tests {
    use crate::data::user_repository;
    use crate::db::establish_db_connection;
    use crate::models::user::{NewUserProfile, UserProfile};
    use crate::schema::user_profile::dsl::*;
    use diesel::prelude::*;
    use diesel::result::Error;

    //generates a dummy user for us
    fn generate_dummy_user_profile() -> NewUserProfile {
        NewUserProfile {
            username: "shadow".to_string(),
            password: "12312".to_string(),
            email: "fake@email.com".to_string(),
            first_name: Some("shadow".to_string()),
            last_name: None,
        }
    }

    // test writing and reading to the user_profile relation
    fn read_user_profile_records(conn: &mut PgConnection) -> QueryResult<Vec<UserProfile>> {
        user_profile
            .select(UserProfile::as_select())
            .load::<UserProfile>(conn)
    }

    #[test]
    fn test_user_profile_write() {
        let connection = &mut establish_db_connection();

        let new_user = generate_dummy_user_profile();

        connection.test_transaction::<_, Error, _>(|conn| {
            let created_user = user_repository::create_user(conn, new_user.clone());

            assert_eq!(created_user.email, new_user.email);

            let res = read_user_profile_records(conn).unwrap();
            assert_eq!(res.len(), 1);

            Ok(())
        });
    }

    #[test]
    fn test_user_profile_update_and_delete() {
        let connection = &mut establish_db_connection();

        let new_user = generate_dummy_user_profile();

        connection.test_transaction::<_, Error, _>(|conn| {
            let created_user = user_repository::create_user(conn, new_user.clone());

            let _username = String::from("sasuga_shadow_sama");
            let _first_name = Some(String::from("Cid"));
            let _last_name = Some(String::from("Kagenou"));

            let updated_user = UserProfile {
                username: _username.clone(),
                first_name: _first_name.clone(),
                last_name: _last_name.clone(),
                ..created_user
            };

            let updated_user = user_repository::update_user_profile_by_id(conn, updated_user);

            //assert whether our values have been updated as expected
            assert_eq!(updated_user.username, _username);
            assert_eq!(updated_user.first_name, _first_name);
            assert_eq!(updated_user.last_name, _last_name);

            let res = read_user_profile_records(conn).unwrap();
            assert_eq!(res.len(), 1);

            Ok(())
        });
    }
}
