use crate::auth::domain::Role;

pub trait RoleTrait {
    fn attach_user(&self, user_id: usize);

    fn detach_user(&self, user_id: usize);

    fn check_user(&self, user_id: usize) -> bool;

    fn get_user_roles(user_id: usize) -> Option<Vec<Role>>;
}
