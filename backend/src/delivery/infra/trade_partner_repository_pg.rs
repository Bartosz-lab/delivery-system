use diesel::prelude::*;

use crate::{
    delivery::domain::{repository::TradePartnerTrait, value_objects::PriceList, TradePartner},
    schema::trade_partners,
    PgPool,
};

#[derive(Clone, Queryable, Identifiable)]
#[diesel(table_name = trade_partners)]
pub struct TradePartnerHelper {
    pub id: i32,
    pub name: String,
    pub price_list: String,
}

impl TradePartnerTrait<PgPool> for TradePartner {
    fn insert(db_pool: PgPool, tradepartner: TradePartner) -> Option<i32> {
        use crate::schema::trade_partners::dsl::*;

        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(trade_partners)
                .values((
                    &name.eq(tradepartner.name),
                    &price_list
                        .eq(serde_json::to_string(&tradepartner.price_list.as_ser_vec()).unwrap()),
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

    fn delete(db_pool: PgPool, tradepartner_id: i32) -> bool {
        use crate::schema::trade_partners::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result =
                diesel::delete(trade_partners.filter(id.eq(tradepartner_id))).execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn save(db_pool: PgPool, tradepartner: TradePartner) -> bool {
        use crate::schema::trade_partners::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::update(&tradepartner.clone())
                .set((
                    name.eq(tradepartner.name),
                    price_list
                        .eq(serde_json::to_string(&tradepartner.price_list.as_ser_vec()).unwrap()),
                ))
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

    fn find_by_id(db_pool: PgPool, tradepartner_id: i32) -> Option<TradePartner> {
        use crate::schema::trade_partners::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = trade_partners
                .filter(id.eq(tradepartner_id))
                .first::<TradePartnerHelper>(&mut conn)
                .optional();

            match result {
                Ok(Some(x)) => Some(TradePartner {
                    id: x.id,
                    name: x.name,
                    price_list: PriceList::from_ser_vec(
                        serde_json::from_str(x.price_list.as_str()).unwrap(),
                    ),
                }),
                Ok(None) => None,
                Err(_) => None, // There should be database error
            }
        } else {
            // There should be database error
            None
        }
    }

    fn get_all(db_pool: PgPool) -> Vec<TradePartner> {
        use crate::schema::trade_partners::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = trade_partners.load::<TradePartnerHelper>(&mut conn);

            if let Ok(res) = result {
                res.into_iter()
                    .map(|x| TradePartner {
                        id: x.id,
                        name: x.name,
                        price_list: PriceList::from_ser_vec(
                            serde_json::from_str(x.price_list.as_str()).unwrap(),
                        ),
                    })
                    .collect()
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
