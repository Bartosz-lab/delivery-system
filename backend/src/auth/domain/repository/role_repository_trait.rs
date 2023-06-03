use crate::auth::domain::Role;

pub trait RoleTrait<T> {
    fn attach_user(&self, db_pool: T, user_id: usize);

    fn detach_user(&self, db_pool: T, user_id: usize);

    fn check_user(&self, db_pool: T, user_id: usize) -> bool;

    fn get_user_roles(db_pool: T, user_id: usize) -> Option<Vec<Role>>;
}
