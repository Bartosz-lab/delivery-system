// use actix_web::{error, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
// use diesel::{prelude::*, r2d2};

// use crate::{
//     auth::domain::{repository::UserTrait, User},
//     PgPool,
// };

// use std::sync::Mutex;

// struct UserRepository {
//     users: Vec<User>,
//     last_id: i32,
// }

// lazy_static! {
//     static ref DATA: Mutex<UserRepository> = {
//         let mut user1 = User::new(
//             "Jan".to_string(),
//             "Testowy".to_string(),
//             "jan@testowy.com".to_string(),
//             "123456789".to_string(),
//         );
//         user1.id = 1;
//         let mut user2 = User::new(
//             "Anna".to_string(),
//             "Testowa".to_string(),
//             "anna@testowa.com".to_string(),
//             "123456789".to_string(),
//         );
//         user2.id = 2;
//         Mutex::new(UserRepository {
//             users: vec![user1, user2],
//             last_id: 10,
//         })
//     };
// }

// impl UserTrait<PgPool> for User {
//     fn insert(_: PgPool, user: User) -> Option<i32> {
//         let mut user = user;
//         let id = DATA.lock().unwrap().last_id;
//         user.id = id;
//         DATA.lock().unwrap().last_id += 1;
//         DATA.lock().unwrap().users.push(user);
//         Some(id)
//     }

//     fn delete(_: PgPool, user_id: i32) -> bool {
//         let _ = &DATA.lock().unwrap().users.retain(|user| user.id != user_id);
//         true
//     }

//     fn save(db_pool: PgPool, user: User) -> bool {
//         User::delete(db_pool, user.id);
//         DATA.lock().unwrap().users.push(user);
//         true
//     }

//     fn find_by_id(db_pool: PgPool, user_id: i32) -> Option<User> {
//         let user = web::block(move || {
//             use crate::schema::users::dsl::*;
//             let mut conn = db_pool.get()?;

//             let user = users
//                 // .filter(id.eq(user_id))
//                 .first::<User>(conn)
//                 .optional()?;
//         })
//         .await?
//         // map diesel query errors to a 500 error response
//         .map_err(error::ErrorInternalServerError)?;

//         let users = &DATA.lock().unwrap().users;

//         let users = users
//             .into_iter()
//             .filter(|user| user.id == id)
//             .collect::<Vec<&User>>();
//         match users.len() {
//             0 => None,
//             _ => Some(users.first().unwrap().clone().clone()),
//         }
//     }

//     fn find_by_email(_: PgPool, email: String) -> Option<User> {
//         let users = &DATA.lock().unwrap().users;

//         let users = users
//             .into_iter()
//             .filter(|user| user.email == email)
//             .collect::<Vec<&User>>();
//         match users.len() {
//             0 => None,
//             _ => Some(users.first().unwrap().clone().clone()),
//         }
//     }
// }
