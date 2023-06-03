use crate::auth::domain::User;

pub trait UserTrait<T> {
    fn insert(db_pool: T, user: User) -> Option<i32>;

    fn save(db_pool: T, user: User) -> bool;

    fn delete(db_pool: T, id: i32) -> bool;

    fn find_by_id(db_pool: T, id: i32) -> Option<User>;

    fn find_by_email(db_pool: T, email: String) -> Option<User>;
}
