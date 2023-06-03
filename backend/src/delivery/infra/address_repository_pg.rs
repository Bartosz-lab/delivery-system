use diesel::prelude::*;

use crate::{
    delivery::domain::{repository::AddressTrait, Address},
    PgPool,
};

impl AddressTrait<PgPool> for Address {
    fn insert(db_pool: PgPool, address: Address) -> Option<i32> {
        use crate::schema::addresses::dsl::*;

        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(addresses)
                .values((
                    &street.eq(address.street),
                    &city.eq(address.city),
                    &postal_code.eq(address.postal_code),
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

    fn delete(db_pool: PgPool, address_id: i32) -> bool {
        use crate::schema::addresses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::delete(addresses.filter(id.eq(address_id))).execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn save(db_pool: PgPool, address: Address) -> bool {
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::update(&address.clone())
                .set(address)
                .get_results::<Address>(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn find_by_id(db_pool: PgPool, address_id: i32) -> Option<Address> {
        use crate::schema::addresses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let address = addresses
                .filter(id.eq(address_id))
                .first::<Address>(&mut conn)
                .optional();

            address.unwrap_or(None) // There should be database error
        } else {
            // There should be database error
            None
        }
    }

    fn get_all(db_pool: PgPool) -> Vec<Address> {
        use crate::schema::addresses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = addresses.load::<Address>(&mut conn);

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
