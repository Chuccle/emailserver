use crate::schema::{Users, Microbits}; 


#[derive(Identifiable, Queryable)]
#[table_name = "Users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset)]
#[belongs_to(parent = "User")]
#[table_name = "Microbits"]
pub struct Microbit {
    pub id: String,
    pub user_id: Option<i32>,
    pub active_begin_hours: i32,
pub active_begin_minutes: i32,
pub active_end_hours: i32,
pub active_end_minutes: i32,
}



#[derive(Insertable)]
#[table_name="Users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}




