use diesel::prelude::*;

use crate::{
    auth::domain::{repository::UserTrait, User},
    PgPool,
};

impl UserTrait<PgPool> for User {
    fn insert(db_pool: PgPool, user: User) -> Option<i32> {
        use crate::schema::users::dsl::*;

        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(users)
                .values((
                    &firstname.eq(user.firstname),
                    &lastname.eq(user.lastname),
                    &email.eq(user.email),
                    &phone.eq(user.phone),
                    &pass_hash.eq(user.pass_hash),
                ))
                .returning(id)
                .get_results(&mut conn);

            if let Ok(id_vec) = result {
                Some(id_vec[0])
            } else {
                None
            }
        } else {
            // There should be database error
            None
        }
    }

    fn delete(db_pool: PgPool, user_id: i32) -> bool {
        use crate::schema::users::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::delete(users.filter(id.eq(user_id))).execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            // There should be database error
            false
        }
    }

    fn save(db_pool: PgPool, user: User) -> bool {
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::update(&user.clone())
                .set(user)
                .get_results::<User>(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            // There should be database error
            false
        }
    }

    fn find_by_id(db_pool: PgPool, user_id: i32) -> Option<User> {
        use crate::schema::users::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let user = users
                .filter(id.eq(user_id))
                .first::<User>(&mut conn)
                .optional();

            user.unwrap_or(None)
        } else {
            // There should be database error
            None
        }
    }

    fn find_by_email(db_pool: PgPool, user_email: String) -> Option<User> {
        use crate::schema::users::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let user = users
                .filter(email.eq(user_email))
                .first::<User>(&mut conn)
                .optional();

            user.unwrap_or(None)
        } else {
            // There should be database error
            None
        }
    }
}
