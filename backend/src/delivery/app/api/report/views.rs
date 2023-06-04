use actix_web::{get, web, HttpResponse, Responder};
use chrono::NaiveDate;

use crate::{
    auth::app::{AdminExtractor, AuthExtractor, CourierExtractor, TradePartnerExtractor},
    delivery::domain::service::{
        parcel_report::{ParcelCollectReport, ParcelDeliveryReport},
        settlement_report::SettlementReport,
    },
    PgPool,
};

type Pool = PgPool;

#[utoipa::path(
    context_path = "/report",
    tag = "Settlement Report",
    responses(
        (status = OK, body = SettlementTotalReport, description = "Settlement Report", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Wrong date format"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/settlement/{start_date}/{end_date}")]
async fn trade_partner_settlement_report(
    db_pool: web::Data<Pool>,
    path: web::Path<(String, String)>,
    _: AuthExtractor,
    extractor: TradePartnerExtractor,
) -> impl Responder {
    let (start_date, end_date) = path.into_inner();
    match (
        NaiveDate::parse_from_str(start_date.as_str(), "%d-%m-%Y"),
        NaiveDate::parse_from_str(end_date.as_str(), "%d-%m-%Y"),
    ) {
        (Ok(start_date), Ok(end_date)) => HttpResponse::Ok().json(SettlementReport::gen_report(
            (**db_pool).clone(),
            start_date,
            end_date,
            extractor.trade_partner_id,
            vec![],
        )),
        (_, _) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/report",
    tag = "Settlement Report",
    responses(
        (status = OK, body = SettlementTotalReport, description = "Settlement Report", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Wrong date format"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/settlement/{start_date}/{end_date}/{trade_partner_id}")]
async fn trade_partner_settlement_report_admin(
    db_pool: web::Data<Pool>,
    path: web::Path<(String, String, i32)>,
    _: AuthExtractor,
    _: AdminExtractor,
) -> impl Responder {
    let (start_date, end_date, trade_partner_id) = path.into_inner();
    match (
        NaiveDate::parse_from_str(start_date.as_str(), "%d-%m-%Y"),
        NaiveDate::parse_from_str(end_date.as_str(), "%d-%m-%Y"),
    ) {
        (Ok(start_date), Ok(end_date)) => HttpResponse::Ok().json(SettlementReport::gen_report(
            (**db_pool).clone(),
            start_date,
            end_date,
            trade_partner_id,
            vec![],
        )),
        (_, _) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/report",
    tag = "Parcel Report",
    responses(
        (status = OK, body = ParcelTotalReport, description = "Parcels to collect Report", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Wrong date format"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/parcel/collect/{date}")]
async fn collect_report(
    db_pool: web::Data<Pool>,
    path: web::Path<String>,
    _: AuthExtractor,
    _: CourierExtractor,
) -> impl Responder {
    let date = path.into_inner();
    match NaiveDate::parse_from_str(date.as_str(), "%d-%m-%Y") {
        Ok(date) => HttpResponse::Ok().json(ParcelCollectReport::gen_report(
            (**db_pool).clone(),
            date,
            vec![],
        )),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/report",
    tag = "Parcel Report",
    responses(
        (status = OK, body = ParcelTotalReport, description = "Parcels to delivery Report", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Wrong date format"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[get("/parcel/delivery/{date}")]
async fn delivery_report(
    db_pool: web::Data<Pool>,
    path: web::Path<String>,
    _: AuthExtractor,
    _: CourierExtractor,
) -> impl Responder {
    let date = path.into_inner();
    match NaiveDate::parse_from_str(date.as_str(), "%d-%m-%Y") {
        Ok(date) => {
            HttpResponse::Ok().json(ParcelDeliveryReport::gen_report((**db_pool).clone(), date))
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
