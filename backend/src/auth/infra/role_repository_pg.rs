use diesel::{dsl::exists, prelude::*, select};

use crate::{
    auth::domain::{repository::RoleTrait, Role},
    PgPool,
};

impl RoleTrait<PgPool> for Role {
    fn attach_user(&self, db_pool: PgPool, arg_user_id: i32) {
        use crate::schema::roles::dsl::*;

        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(roles)
                .values((
                    &role_name.eq(serde_json::to_string(self).unwrap()),
                    &user_id.eq(arg_user_id),
                ))
                .execute(&mut conn);

            if let Err(_) = result {
                // There should be database error
            }
        } else {
            // There should be database error
        }
    }

    fn detach_user(&self, db_pool: PgPool, arg_user_id: i32) {
        use crate::schema::roles::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::delete(
                roles.filter(
                    role_name
                        .eq(serde_json::to_string(self).unwrap())
                        .and(user_id.eq(arg_user_id)),
                ),
            )
            .execute(&mut conn);

            if let Err(_) = result {
                // There should be database error
            }
        } else {
            // There should be database error
        }
    }

    fn check_user(&self, db_pool: PgPool, arg_user_id: i32) -> bool {
        use crate::schema::roles::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = select(exists(
                roles.filter(
                    role_name
                        .eq(serde_json::to_string(self).unwrap())
                        .and(user_id.eq(arg_user_id)),
                ),
            ))
            .get_result(&mut conn);

            match result {
                Ok(res) => res,
                Err(_) => false,
            }
        } else {
            // There should be database error
            false
        }
    }

    fn get_user_roles(db_pool: PgPool, arg_user_id: i32) -> Option<Vec<Role>> {
        use crate::schema::roles::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = roles
                .filter(user_id.eq(arg_user_id))
                .select(role_name)
                .load::<String>(&mut conn);

            if let Ok(res) = result {
                Some(
                    res.into_iter()
                        .map(|x| {
                            let tmp: Role = serde_json::from_str(&x).unwrap();
                            tmp
                        })
                        .collect::<Vec<Role>>(),
                )
            } else {
                None
            }
        } else {
            // There should be database error
            None
        }
    }
}
