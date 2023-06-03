use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::{
    auth::{
        app::{
            api::users::structs::{AddUser, UserBody},
            {AdminExtractor, AuthExtractor},
        },
        domain::{repository::UserTrait, User},
    },
    IMPool,
};

type Pool = IMPool;

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    request_body(content = AddUser,
        content_type = "application/json", 
        description = "Create new user",
    ),
    responses(
        (status = CREATED, body = usize, description = "User created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "User not created due to invalid data"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("")]
async fn view_admin_add_user(
    db_pool: web::Data<Pool>,
    body: web::Json<AddUser>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let res = User::insert(
        **db_pool,
        User::new(
            body.firstname.to_owned(),
            body.lastname.to_owned(),
            body.email.to_owned(),
            body.phone.to_owned(),
        ),
    );
    match res {
        Some(id) => HttpResponse::Created().json(id),
        None => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    request_body(content = UserBody,
        content_type = "application/json", 
        description = "Modify user",
    ),
    responses(
        (status = OK, description = "User modified successfully"),
        (status = BAD_REQUEST, description = "Data validation fail"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
    )
)]
#[put("")]
async fn view_modify_user(
    db_pool: web::Data<Pool>,
    body: web::Json<UserBody>,
    auth: AuthExtractor,
) -> impl Responder {
    modify_user(**db_pool, auth.user.user_id, body)
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    responses(
        (status = OK, body = UserBody, description = "User data", content_type = "application/json"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
    )
)]
#[get("")]
async fn view_get_user(db_pool: web::Data<Pool>, auth: AuthExtractor) -> impl Responder {
    match User::find_by_id(**db_pool, auth.user.user_id) {
        None => HttpResponse::InternalServerError().finish(),
        Some(user) => HttpResponse::Ok().json(UserBody {
            firstname: Some(user.firstname),
            lastname: Some(user.lastname),
            email: Some(user.email),
            phone: Some(user.phone),
        }),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    responses(
        (status = OK, body = UserBody, description = "User data", content_type = "application/json"),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/{user_id}")]
async fn view_admin_get_user(
    db_pool: web::Data<Pool>,
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match User::find_by_id(**db_pool, user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(user) => HttpResponse::Ok().json(UserBody {
            firstname: Some(user.firstname),
            lastname: Some(user.lastname),
            email: Some(user.email),
            phone: Some(user.phone),
        }),
    }
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    request_body(content = UserBody,
        content_type = "application/json", 
        description = "Modify user",
    ),
    responses(
        (status = OK, description = "User modified successfully"),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = BAD_REQUEST, description = "Data validation fail"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[put("/{user_id}")]
async fn view_admin_modify_user(
    db_pool: web::Data<Pool>,
    path: web::Path<usize>,
    body: web::Json<UserBody>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    modify_user(**db_pool, user_id, body)
}

#[utoipa::path(
    context_path = "/user",
    tag = "User",
    responses(
        (status = CREATED, description = "User deleted successfully"),
        (status = NOT_FOUND, description = "User don't exist"),
        (status = BAD_REQUEST, description = "Can't delete user"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[delete("/{user_id}")]
async fn view_admin_delete_user(
    db_pool: web::Data<Pool>,
    path: web::Path<usize>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let user_id = path.into_inner();
    match User::find_by_id(**db_pool, user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(user) => {
            if User::delete(**db_pool, user.id) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

fn modify_user(db_pool: Pool, user_id: usize, body: web::Json<UserBody>) -> impl Responder {
    match User::find_by_id(db_pool, user_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut user) => {
            // There should be data validation
            if let Some(firstname) = &body.firstname {
                user.firstname = firstname.clone();
            }
            if let Some(lastname) = &body.lastname {
                user.lastname = lastname.clone();
            }
            if let Some(email) = &body.email {
                user.email = email.clone();
            }
            if let Some(phone) = &body.phone {
                user.phone = phone.clone();
            }
            if User::save(db_pool, user) {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}
