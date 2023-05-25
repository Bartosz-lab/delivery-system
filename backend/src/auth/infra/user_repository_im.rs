use crate::auth::domain::repository::UserTrait;
use crate::auth::domain::User;

use std::sync::Mutex;

struct UserRepository {
    users: Vec<User>,
    last_id: usize,
}

lazy_static! {
    static ref DATA: Mutex<UserRepository> = {
        let mut user1 = User::new(
            "Jan".to_string(),
            "Testowy".to_string(),
            "jan@testowy.com".to_string(),
            "123456789".to_string(),
        );
        user1.id = 1;
        let mut user2 = User::new(
            "Anna".to_string(),
            "Testowa".to_string(),
            "anna@testowa.com".to_string(),
            "123456789".to_string(),
        );
        user2.id = 2;
        Mutex::new(UserRepository {
            users: vec![user1, user2],
            last_id: 10,
        })
    };
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

    fn delete(user_id: usize) -> bool {
        let _ = &DATA.lock().unwrap().users.retain(|user| user.id != user_id);
        true
    }

    fn save(user: User) -> bool {
        User::delete(user.id);
        DATA.lock().unwrap().users.push(user);
        true
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
