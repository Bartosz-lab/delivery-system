use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

// use crate::auth::app::{AdminExtractor, AuthExtractor};
use crate::delivery::app::api::parcel::structs::{AddressBody, ParcelBody};
use crate::delivery::domain::repository::{AddressTrait, ParcelTrait};
use crate::delivery::domain::{Address, Parcel};

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
                recipient_address: AddressBody {
                    street: address.street,
                    city: address.city,
                    postal_code: address.postal_code,
                },
            })
        }
    }
}
