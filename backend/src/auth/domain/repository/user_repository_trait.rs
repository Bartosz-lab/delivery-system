use crate::auth::domain::User;

pub trait UserTrait {
    fn insert(user: User) -> Option<usize>;

    fn save(user: User) -> bool;

    fn delete(id: usize) -> bool;

    fn find_by_id(id: usize) -> Option<User>;

    fn find_by_email(email: String) -> Option<User>;
}
