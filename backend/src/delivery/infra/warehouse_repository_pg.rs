use diesel::prelude::*;

use crate::{
    delivery::domain::{repository::WarehouseTrait, Warehouse},
    PgPool,
};

impl WarehouseTrait<PgPool> for Warehouse {
    fn insert(db_pool: PgPool, warehouse: Warehouse) -> Option<i32> {
        use crate::schema::warehouses::dsl::*;

        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(warehouses)
                .values((
                    &name.eq(warehouse.name),
                    &trade_partner_id.eq(warehouse.trade_partner_id),
                    &address_id.eq(warehouse.address_id),
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

    fn delete(db_pool: PgPool, warehouse_id: i32) -> bool {
        use crate::schema::warehouses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::delete(warehouses.filter(id.eq(warehouse_id))).execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn save(db_pool: PgPool, warehouse: Warehouse) -> bool {
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::update(&warehouse.clone())
                .set(warehouse)
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

    fn find_by_id(db_pool: PgPool, warehouse_id: i32) -> Option<Warehouse> {
        use crate::schema::warehouses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let warehouse = warehouses
                .filter(id.eq(warehouse_id))
                .first::<Warehouse>(&mut conn)
                .optional();

            warehouse.unwrap_or(None) // There should be database error
        } else {
            // There should be database error
            None
        }
    }

    fn find_by_trade_partner(db_pool: PgPool, arg_trade_partner_id: i32) -> Vec<Warehouse> {
        use crate::schema::warehouses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = warehouses
                .filter(trade_partner_id.eq(arg_trade_partner_id))
                .load::<Warehouse>(&mut conn);

            if let Ok(res) = result {
                res.into_iter()
                    .enumerate()
                    .map(|(tp_id, warehouse)| {
                        let mut new = warehouse.clone();
                        new.id = tp_id as i32;
                        new.clone()
                    })
                    .collect::<Vec<Warehouse>>()
            } else {
                // There should be database error
                Vec::new()
            }
        } else {
            // There should be database error
            Vec::new()
        }
    }

    fn find_by_trade_partner_and_id(
        db_pool: PgPool,
        arg_trade_partner_id: i32,
        warehouse_id: i32,
    ) -> Option<Warehouse> {
        use crate::schema::warehouses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = warehouses
                .filter(trade_partner_id.eq(arg_trade_partner_id))
                .order(id.asc())
                .limit(1)
                .offset(warehouse_id.into())
                .first::<Warehouse>(&mut conn)
                .optional();

            result.unwrap_or(None)
        } else {
            // There should be database error
            None
        }
    }

    fn get_all(db_pool: PgPool) -> Vec<Warehouse> {
        use crate::schema::warehouses::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = warehouses.load::<Warehouse>(&mut conn);

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
