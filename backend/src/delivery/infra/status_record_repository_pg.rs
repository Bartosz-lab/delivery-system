use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    delivery::domain::{repository::StatusRecordTrait, value_objects::ParcelStatus, StatusRecord},
    schema::status_records,
    PgPool,
};

#[derive(Clone, Queryable, Identifiable)]
#[diesel(table_name = status_records)]
pub struct StatusRecordHelper {
    pub id: i32,
    pub parcel_id: i32,
    pub status: String,
    pub creation_time: NaiveDateTime,
}

impl StatusRecordTrait<PgPool> for StatusRecord {
    fn insert(db_pool: PgPool, status_record: StatusRecord) -> Option<i32> {
        use crate::schema::status_records::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::insert_into(status_records)
                .values((
                    &parcel_id.eq(status_record.parcel_id),
                    &status.eq(serde_json::to_string(&status_record.status).unwrap()),
                    &creation_time.eq(status_record.creation_time),
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

    fn delete(db_pool: PgPool, status_record_id: i32) -> bool {
        use crate::schema::status_records::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result =
                diesel::delete(status_records.filter(id.eq(status_record_id))).execute(&mut conn);

            match result {
                Ok(_) => true,
                Err(_) => false, // There should be database error
            }
        } else {
            // There should be database error
            false
        }
    }

    fn save(db_pool: PgPool, status_record: StatusRecord) -> bool {
        use crate::schema::status_records::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = diesel::update(&status_record.clone())
                .set((
                    parcel_id.eq(status_record.parcel_id),
                    status.eq(serde_json::to_string(&status_record.status).unwrap()),
                    creation_time.eq(status_record.creation_time),
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

    fn find_by_id(db_pool: PgPool, status_record_id: i32) -> Option<StatusRecord> {
        use crate::schema::status_records::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = status_records
                .filter(id.eq(status_record_id))
                .first::<StatusRecordHelper>(&mut conn)
                .optional();

            match result {
                Ok(Some(x)) => Some(StatusRecord {
                    id: x.id,
                    parcel_id: x.parcel_id,
                    status: serde_json::from_str(x.status.as_str()).unwrap(),
                    creation_time: x.creation_time,
                }),
                Ok(None) => None,
                Err(_) => None, // There should be database error
            }
        } else {
            // There should be database error
            None
        }
    }

    fn find_by_parcel_id(db_pool: PgPool, arg_parcel_id: i32) -> Vec<StatusRecord> {
        use crate::schema::status_records::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = status_records
                .filter(parcel_id.eq(arg_parcel_id))
                .load::<StatusRecordHelper>(&mut conn);

            if let Ok(res) = result {
                res.into_iter()
                    .map(|x| StatusRecord {
                        id: x.id,
                        parcel_id: x.parcel_id,
                        status: serde_json::from_str(x.status.as_str()).unwrap(),
                        creation_time: x.creation_time,
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

    fn find_by_status(db_pool: PgPool, arg_status: ParcelStatus) -> Vec<StatusRecord> {
        use crate::schema::status_records::dsl::*;
        if let Ok(mut conn) = db_pool.get() {
            let result = status_records
                .filter(status.eq(serde_json::to_string(&arg_status).unwrap()))
                .load::<StatusRecordHelper>(&mut conn);

            if let Ok(res) = result {
                res.into_iter()
                    .map(|x| StatusRecord {
                        id: x.id,
                        parcel_id: x.parcel_id,
                        status: serde_json::from_str(x.status.as_str()).unwrap(),
                        creation_time: x.creation_time,
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
