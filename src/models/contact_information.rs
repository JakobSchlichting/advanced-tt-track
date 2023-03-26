use diesel::{Insertable, Queryable, Selectable};
use crate::schema::contact_informations;

#[derive(Debug, Selectable, Queryable, Insertable)]
pub struct ContactInformation {
    id: i32,
    email: String,
    phone: Option<String>
}
