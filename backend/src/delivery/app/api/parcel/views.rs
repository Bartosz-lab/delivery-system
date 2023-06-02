use actix_web::{get, post, put, web, web::Json, HttpResponse, Responder};
use chrono::NaiveDate;

use crate::auth::app::{AuthExtractor, CourierExtractor, TradePartnerApiExtractor};
use crate::delivery::app::api::parcel::structs::{
    ModifyParcelRequest, ParcelBody, ParcelRequest, StatusBody,
};
use crate::delivery::app::api::structs::AddressRequired;
use crate::delivery::domain::repository::{
    AddressTrait, ParcelTrait, StatusRecordTrait, WarehouseTrait,
};
use crate::delivery::domain::{
    value_objects::ParcelStatus, Address, Parcel, StatusRecord, Warehouse,
};

#[utoipa::path(
    context_path = "/parcel",
    tag = "Free Access",
    responses(
        (status = OK, body = ParcelBody, description = "Parcel data", content_type = "application/json"),
        (status = NOT_FOUND, description = "Parcel don't exist"),
    )
)]
#[get("/{parcel_id}")]
async fn get_parcel(path: web::Path<usize>) -> impl Responder {
    let parcel_id = path.into_inner();
    match Parcel::find_by_id(parcel_id) {
        None => HttpResponse::NotFound().finish(),
        Some(parcel) => {
            let address = Address::find_by_id(parcel.recipient_address_id).unwrap();
            HttpResponse::Ok().json(ParcelBody {
                recipient_name: parcel.recipient_name,
                recipient_email: parcel.recipient_email,
                recipient_phone: parcel.recipient_phone,
                pickup_date: parcel.pickup_date.format("%d-%m-%Y").to_string(),
                size: parcel.size,
                recipient_address: AddressRequired {
                    street: address.street,
                    city: address.city,
                    postal_code: address.postal_code,
                },
                status_list: StatusRecord::find_by_parcel_id(parcel_id)
                    .into_iter()
                    .map(|status_record| StatusBody {
                        status: status_record.status,
                        time: status_record
                            .creation_time
                            .format("%d-%m-%Y %H:%M")
                            .to_string(),
                    })
                    .collect(),
            })
        }
    }
}

#[utoipa::path(
    context_path = "/parcel",
    tag = "Trade Partner API",
    request_body(content = ParcelRequest,
        content_type = "application/json",
        description = "Add new Parcel",
    ),
    responses(
        (status = CREATED, body = usize, description = "Parcel created successfully", content_type = "application/json"),
        (status = BAD_REQUEST, description = "Parcel not created due to invalid data"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("")]
async fn add_parcel(
    body: Json<ParcelRequest>,
    _: AuthExtractor,
    extractor: TradePartnerApiExtractor,
) -> impl Responder {
    // There should be data validation
    match (
        NaiveDate::parse_from_str(body.pickup_date.as_str(), "%d-%m-%Y"),
        Warehouse::find_by_trade_partner_and_id(extractor.trade_partner_id, body.warehouse_id),
        Address::insert(Address::new(
            body.recipient_address.street.clone(),
            body.recipient_address.city.clone(),
            body.recipient_address.postal_code.clone(),
        )),
    ) {
        (Ok(date), Some(warehouse), Some(address_id)) => match Parcel::insert(Parcel::new(
            body.recipient_name.clone(),
            body.recipient_email.clone(),
            body.recipient_phone.clone(),
            address_id,
            warehouse.id,
            date,
            body.size.clone(),
        )) {
            None => HttpResponse::BadRequest().finish(),
            Some(id) => HttpResponse::Created().json(id),
        },
        (_, _, _) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    context_path = "/parcel",
    tag = "Parcel Status",
    request_body(content = ParcelStatus,
        content_type = "application/json",
        description = "Add new Parcel",
    ),
    responses(
        (status = OK, description = "Status added successfully"),
        (status = BAD_REQUEST, description = "Status not added due to invalid data"),
        (status = UNAUTHORIZED, description = "User isn't logged in"),
        (status = FORBIDDEN, description = "User don't have permissions"),
    )
)]
#[post("/{parcel_id}/status")]
async fn courier_add_status(
    path: web::Path<usize>,
    parcel_status: Json<ParcelStatus>,
    _: AuthExtractor,
    _: CourierExtractor,
) -> impl Responder {
    let parcel_id = path.into_inner();
    match StatusRecord::insert(StatusRecord::new(parcel_id, parcel_status.0)) {
        None => HttpResponse::BadRequest().finish(),
        Some(_) => HttpResponse::Ok().finish(),
    }
}

#[utoipa::path(
    context_path = "/parcel",
    tag = "Free Access",
    request_body(content = ModifyParcelRequest,
        content_type = "application/json",
        description = "Modify Parcel",
    ),
    responses(
        (status = OK, description = "Parcel modified succesfully"),
        (status = BAD_REQUEST, description = "Parcel not modified due to invalid data"),
        (status = NOT_FOUND, description = "Parcel don't exist"),
    )
)]
#[put("/{parcel_id}")]
async fn modify_parcel(path: web::Path<usize>, body: Json<ModifyParcelRequest>) -> impl Responder {
    let parcel_id = path.into_inner();
    match Parcel::find_by_id(parcel_id) {
        None => HttpResponse::NotFound().finish(),
        Some(mut parcel) => {
            if let Some(address) = &body.address {
                match Address::insert(Address::new(
                    address.street.clone(),
                    address.city.clone(),
                    address.postal_code.clone(),
                )) {
                    None => return HttpResponse::BadRequest().finish(),
                    Some(address_id) => {
                        match StatusRecord::insert(StatusRecord::new(
                            parcel_id,
                            ParcelStatus::ChangedAddress(parcel.recipient_address_id),
                        )) {
                            None => return HttpResponse::BadRequest().finish(),
                            Some(_) => parcel.recipient_address_id = address_id,
                        }
                        if !Parcel::save(parcel) {
                            return HttpResponse::BadRequest().finish();
                        }
                    }
                }
            }
            if let Some(requested_date) = &body.requested_date {
                if StatusRecord::insert(StatusRecord::new(
                    parcel_id,
                    ParcelStatus::RequestedDelivery(requested_date.clone()),
                ))
                .is_none()
                {
                    return HttpResponse::BadRequest().finish();
                }
            }
            HttpResponse::Ok().finish()
        }
    }
}
