use chrono::NaiveDate;
use diesel::prelude::*;

use crate::{
    delivery::domain::{repository::ParcelTrait, value_objects::ParcelSize, Parcel},
    PgPool,
};

impl ParcelTrait<PgPool> for Parcel {
    fn insert(db_pool: PgPool, parcel: Parcel) -> Option<i32> {
        use crate::schema::parcels::dsl::*;

        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(parcels)
                .values((
                    &id.eq(parcel.id),
                    &recipient_name.eq(parcel.recipient_name),
                    &recipient_email.eq(parcel.recipient_email),
                    &recipient_phone.eq(parcel.recipient_phone),
                    &recipient_address_id.eq(parcel.recipient_address_id),
                    &warehouse_id.eq(parcel.warehouse_id),
                    &pickup_date.eq(parcel.pickup_date),
                    &size.eq(parcel.size),
                ))
                .returning(id)
                .get_results(&mut conn);

            if let Ok(id_vec) = result {
                Some(id_vec[0])
            } else if let Err(err) = result {
                println!("Error: {:?}", err);
                None
            } else {
                None
            }
        } else {
            // There should be database error
            None
        }
    }

    fn delete(db_pool: PgPool, parcel_id: i32) -> bool {
        use crate::schema::parcels::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::delete(parcels.filter(id.eq(parcel_id))).execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn save(db_pool: PgPool, parcel: Parcel) -> bool {
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::update(&parcel.clone())
                .set(parcel)
                .execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn find_by_id(db_pool: PgPool, parcel_id: i32) -> Option<Parcel> {
        use crate::schema::parcels::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let parcel = parcels
                .filter(id.eq(parcel_id))
                .first::<Parcel>(&mut conn)
                .optional();

            parcel.unwrap_or(None) // There should be database error
        } else {
            // There should be database error
            None
        }
    }

    fn find_by_warehouse_id(db_pool: PgPool, arq_warehouse_id: i32) -> Vec<Parcel> {
        use crate::schema::parcels::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = parcels
                .filter(warehouse_id.eq(arq_warehouse_id))
                .load::<Parcel>(&mut conn);

            if let Ok(res) = result {
                res
            } else {
                // There should be database error
                Vec::new()
            }
        } else {
            // There should be database error
            Vec::new()
        }
    }

    fn find_by_date_and_warehouse_id(
        db_pool: PgPool,
        start_date: NaiveDate,
        end_date: NaiveDate,
        arq_warehouse_id: i32,
        arg_size: ParcelSize,
    ) -> Vec<Parcel> {
        use crate::schema::parcels::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = parcels
                .filter(
                    warehouse_id
                        .eq(arq_warehouse_id)
                        .and(pickup_date.between(start_date, end_date))
                        .and(size.eq(arg_size)),
                )
                .load::<Parcel>(&mut conn);

            if let Ok(res) = result {
                res
            } else {
                // There should be database error
                Vec::new()
            }
        } else {
            // There should be database error
            Vec::new()
        }
    }
}
