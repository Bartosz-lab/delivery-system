use crate::auth::domain::Role;

pub trait RoleTrait<T> {
    fn attach_user(&self, db_pool: T, user_id: i32);

    fn detach_user(&self, db_pool: T, user_id: i32);

    fn check_user(&self, db_pool: T, user_id: i32) -> bool;

    fn get_user_roles(db_pool: T, user_id: i32) -> Option<Vec<Role>>;
}
