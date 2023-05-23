use std::collections::HashSet;

use super::role::Role;

pub struct User {
    id: usize,
    pub firstname: String,
    pub lastname: String,
    email: String,
    pub phone: String,
    roles: HashSet<Role>,
}

impl User {
    pub fn new(firstname: String, lastname: String, email: String, phone: String) -> Self {
        User {
            id: 1,
            firstname,
            lastname,
            email,
            phone,
            roles: HashSet::new(),
        }
    }

    /// Get id
    pub fn id(&self) -> &usize {
        &self.id
    }

    /// Get email
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get roles
    pub fn roles(&self) -> Vec<Role> {
        self.roles.clone().into_iter().collect::<Vec<Role>>()
    }

    /// Attach new role to the user
    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role);
    }

    /// Detach role from the user
    pub fn del_role(&mut self, role: Role) {
        self.roles.insert(role);
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            firstname: "Jan".to_string(),
            lastname: "Testowy".to_string(),
            email: "jan@testowy.com".to_string(),
            phone: "123456789".to_string(),
            roles: HashSet::new(),
        }
    }
}
