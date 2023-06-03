use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pass_hash: String,
}

impl User {
    pub fn new(firstname: String, lastname: String, email: String, phone: String) -> Self {
        let password = "Pass1234".to_owned(); // this line should be changed to autogen pass send by mail
        User {
            id: 0,
            firstname,
            lastname,
            email,
            phone,
            pass_hash: User::gen_hash(password),
        }
    }

    pub fn set_password(&mut self, pass: String) {
        self.pass_hash = User::gen_hash(pass);
    }

    pub fn check_password(&self, pass: String) -> bool {
        let parsed_hash = PasswordHash::new(&self.pass_hash).unwrap();
        Argon2::default()
            .verify_password(pass.as_bytes(), &parsed_hash)
            .is_ok()
    }

    fn gen_hash(pass: String) -> String {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(pass.as_bytes(), &salt)
            .expect("Error while hashing password")
            .to_string()
    }
}
