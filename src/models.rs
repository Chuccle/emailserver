#[derive(Queryable)]
pub struct User {
    pub accountID: i32,
    pub Email: String,
    pub Password: String,
}

#[derive(Queryable)]
pub struct Microbit {
    pub MicrobitID: i32,
    pub F_AccountID: Option<i32>,
}



use super::schema::Users;

#[derive(Insertable)]
#[table_name="Users"]
pub struct NewUser<'a> {
    pub Email: &'a str,
    pub Password: &'a str,
}