use chrono::NaiveDate;

use crate::{
    delivery::domain::{
        repository::{AddressTrait, ParcelTrait, StatusRecordTrait, WarehouseTrait},
        value_objects::{ParcelSize, ParcelStatus},
        Address, Parcel, StatusRecord, Warehouse,
    },
    IMPool,
};

type Pool = IMPool;

pub mod structs;
use structs::{
    AddressBody, DeliveryReport, ParcelBody, ParcelSizeReport, ParcelTotalReport,
    ParcelWarehouseReport,
};

pub struct ParcelCollectReport;

impl ParcelCollectReport {
    pub fn gen_report(
        db_pool: Pool,
        date: NaiveDate,
        warehouses_id: Vec<i32>,
    ) -> ParcelTotalReport {
        let mut warehouses_id = warehouses_id;
        if warehouses_id.len() == 0 {
            warehouses_id = Warehouse::get_all(db_pool)
                .into_iter()
                .map(|warehouse| warehouse.id)
                .collect()
        }

        let warehouse_reports = ParcelCollectReport::gen_warehouses(db_pool, date, warehouses_id);

        let parcels_num = warehouse_reports
            .clone()
            .into_iter()
            .map(|report| report.parcels_num)
            .sum();

        ParcelTotalReport {
            date: date.format("%d-%m-%Y").to_string(),
            warehouses: warehouse_reports,
            parcels_num,
        }
    }

    fn gen_warehouses(
        db_pool: Pool,
        date: NaiveDate,
        warehouses_id: Vec<i32>,
    ) -> Vec<ParcelWarehouseReport> {
        warehouses_id
            .into_iter()
            .map(|warehouse_id| {
                let size_reports = ParcelCollectReport::gen_sizes(db_pool, date, warehouse_id);

                let parcels_num = size_reports
                    .clone()
                    .into_iter()
                    .map(|report| report.parcels_num)
                    .sum();

                ParcelWarehouseReport {
                    warehouse_id,
                    sizes: size_reports,
                    parcels_num,
                }
            })
            .collect()
    }

    fn gen_sizes(db_pool: Pool, date: NaiveDate, warehouse_id: i32) -> Vec<ParcelSizeReport> {
        ParcelSize::iterator()
            .map(|size| {
                let parcels =
                    Parcel::find_by_date_and_warehouse_id(db_pool, date, date, warehouse_id, size);
                let parcels_num = parcels.len();

                ParcelSizeReport {
                    size,
                    parcels_id: parcels.into_iter().map(|parcel| parcel.id).collect(),
                    parcels_num,
                }
            })
            .collect()
    }
}

pub struct ParcelDeliveryReport;

impl ParcelDeliveryReport {
    pub fn gen_report(db_pool: Pool, date: NaiveDate) -> DeliveryReport {
        let parcels = StatusRecord::find_by_status(
            db_pool,
            ParcelStatus::ExpectedDelivery(date.format("%d-%m-%Y").to_string()),
        )
        .into_iter()
        .map(|status_record| status_record.parcel_id)
        .collect::<Vec<i32>>();

        let parcels_num = parcels.len();

        let parcels = parcels
            .into_iter()
            .filter_map(|id| {
                let parcel = Parcel::find_by_id(db_pool, id);
                if parcel.is_none() {
                    return None;
                }
                let parcel = parcel.unwrap();

                let address = Address::find_by_id(db_pool, parcel.recipient_address_id);
                if address.is_none() {
                    return None;
                }
                let address = address.unwrap();

                Some(ParcelBody {
                    recipient_name: parcel.recipient_name,
                    recipient_email: parcel.recipient_email,
                    recipient_phone: parcel.recipient_phone,
                    recipient_address: AddressBody {
                        street: address.street,
                        city: address.city,
                        postal_code: address.postal_code,
                    },
                    size: parcel.size,
                })
            })
            .collect();

        DeliveryReport {
            date: date.format("%d-%m-%Y").to_string(),
            parcels,
            parcels_num,
        }
    }
}
