#[derive(Clone)]
pub struct User {
    pub id: usize,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
}

impl User {
    pub fn new(firstname: String, lastname: String, email: String, phone: String) -> Self {
        User {
            id: 0,
            firstname,
            lastname,
            email,
            phone,
        }
    }
}
