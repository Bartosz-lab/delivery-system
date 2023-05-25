use crate::auth::domain::repository::RoleTrait;
use crate::auth::domain::Role;

use std::sync::Mutex;

#[derive(Debug, PartialEq, Clone)]
struct RoleRow {
    role: Role,
    user_id: usize,
}

struct RoleRepository {
    roles: Vec<RoleRow>,
}

lazy_static! {
    static ref DATA: Mutex<RoleRepository> = {
        Mutex::new(RoleRepository {
            roles: vec![
                RoleRow {
                    role: Role::Admin,
                    user_id: 1,
                },
                RoleRow {
                    role: Role::Admin,
                    user_id: 2,
                },
            ],
        })
    };
}

impl RoleTrait for Role {
    fn attach_user(&self, user_id: usize) {
        DATA.lock().unwrap().roles.push(RoleRow {
            role: self.clone(),
            user_id,
        });
        println!("{:?}", DATA.lock().unwrap().roles);
    }

    fn detach_user(&self, user_id: usize) {
        DATA.lock()
            .unwrap()
            .roles
            .retain(|role_row| role_row.user_id == user_id && role_row.role == self.clone())
    }

    fn check_user(&self, user_id: usize) -> bool {
        DATA.lock().unwrap().roles.contains(&RoleRow {
            role: self.clone(),
            user_id,
        })
    }

    fn get_user_roles(user_id: usize) -> Vec<Role> {
        DATA.lock()
            .unwrap()
            .roles
            .clone()
            .into_iter()
            .filter(|role_row| role_row.user_id == user_id)
            .map(|role_row| role_row.role)
            .collect::<Vec<Role>>()
    }
}
