use crate::models::contact::{Contact, NewContact};
use crate::schema::contact;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_contact(conn: &mut PgConnection, new_contact: NewContact) -> Contact {
    diesel::insert_into(contact::table)
        .values(&new_contact)
        .returning(Contact::as_returning())
        .get_result(conn)
        .expect("error inserting new contact record into database")
}
