use crate::auth::domain::repository::UserTrait;
use crate::auth::domain::User;

use std::sync::Mutex;

struct UserRepository {
    users: Vec<User>,
    last_id: usize,
}

lazy_static! {
    static ref DATA: Mutex<UserRepository> = Mutex::new(UserRepository {
        users: vec![
            User {
                id: 1,
                firstname: "Jan".to_string(),
                lastname: "Testowy".to_string(),
                email: "jan@testowy.com".to_string(),
                phone: "123456789".to_string(),
            },
            User {
                id: 2,
                firstname: "Anna".to_string(),
                lastname: "Testowa".to_string(),
                email: "anna@testowa.com".to_string(),
                phone: "123456789".to_string(),
            }
        ],
        last_id: 10,
    });
}

impl UserTrait for User {
    fn insert(user: User) -> Option<usize> {
        let mut user = user;
        let id = DATA.lock().unwrap().last_id;
        user.id = id;
        DATA.lock().unwrap().last_id += 1;
        DATA.lock().unwrap().users.push(user);
        Some(id)
    }

    fn delete(_id: usize) -> bool {
        false
    }

    fn find_by_id(id: usize) -> Option<User> {
        let users = &DATA.lock().unwrap().users;

        let users = users
            .into_iter()
            .filter(|user| user.id == id)
            .collect::<Vec<&User>>();
        match users.len() {
            0 => None,
            _ => Some(users.first().unwrap().clone().clone()),
        }
    }

    fn find_by_email(email: String) -> Option<User> {
        let users = &DATA.lock().unwrap().users;

        let users = users
            .into_iter()
            .filter(|user| user.email == email)
            .collect::<Vec<&User>>();
        match users.len() {
            0 => None,
            _ => Some(users.first().unwrap().clone().clone()),
        }
    }
}
