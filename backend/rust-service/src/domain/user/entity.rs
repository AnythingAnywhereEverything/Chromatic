use crate::domain::user::types::{Email, Username};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: Email,
    pub username: Option<Username>,
    // pub phone_number: Option<String>,
}

impl User {
    pub fn set_username(&mut self, username: Username) -> Result<(), String> {
        self.username = Some(username);
        Ok(())
    }

    pub fn set_email(&mut self, email: Email) -> Result<(), String> {
        self.email = email;
        Ok(())
    }
}