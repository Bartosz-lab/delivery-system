use chrono::NaiveDate;
use rust_decimal::Decimal;
use rusty_money::iso;

use crate::{
    delivery::domain::{
        repository::{ParcelTrait, TradePartnerTrait, WarehouseTrait},
        value_objects::{ParcelSize, PriceList},
        Parcel, TradePartner, Warehouse,
    },
    PgPool,
};

type Pool = PgPool;

pub mod structs;
use structs::{MoneyBody, SettlementSizeReport, SettlementTotalReport, SettlementWarehouseReport};

pub struct SettlementReport;

impl SettlementReport {
    pub fn gen_report(
        db_pool: Pool,
        start_date: NaiveDate,
        end_date: NaiveDate,
        trade_partner_id: i32,
        warehouses_id: Vec<i32>,
    ) -> Option<SettlementTotalReport> {
        let mut warehouses_id = warehouses_id;
        if warehouses_id.len() == 0 {
            warehouses_id = Warehouse::find_by_trade_partner(db_pool.clone(), trade_partner_id)
                .into_iter()
                .map(|warehouse| warehouse.id)
                .collect()
        }

        let trade_partner = TradePartner::find_by_id(db_pool.clone(), trade_partner_id);
        if trade_partner.is_none() {
            return None;
        }
        let trade_partner = trade_partner.unwrap();

        let warehouses_id = warehouses_id
            .into_iter()
            .filter_map(|warehouse_id| {
                let warehouse = Warehouse::find_by_trade_partner_and_id(
                    db_pool.clone(),
                    trade_partner_id,
                    warehouse_id,
                );
                if warehouse.is_none() {
                    return None;
                }
                Some((warehouse.unwrap().id, warehouse_id))
            })
            .collect();

        let warehouse_reports = SettlementReport::gen_warehouses(
            db_pool,
            trade_partner.price_list,
            start_date,
            end_date,
            warehouses_id,
        );

        let warehouse_reports_vec = warehouse_reports
            .clone()
            .into_iter()
            .map(|(report, _)| report)
            .collect::<Vec<SettlementWarehouseReport>>();

        let price_sum: Decimal = warehouse_reports
            .clone()
            .into_iter()
            .map(|(_, price)| price.0.clone())
            .sum();

        let currency = warehouse_reports
            .into_iter()
            .map(|(_, price)| price.1)
            .next()
            .unwrap_or(*iso::PLN);

        let parcels_num = warehouse_reports_vec
            .clone()
            .into_iter()
            .map(|report| report.parcels_num)
            .sum();

        Some(SettlementTotalReport {
            start_date: start_date.format("%d-%m-%Y").to_string(),
            end_date: end_date.format("%d-%m-%Y").to_string(),
            warehouses: warehouse_reports_vec,
            price: MoneyBody {
                price: price_sum.to_string(),
                currency: currency.to_string(),
            },
            parcels_num,
        })
    }

    fn gen_warehouses(
        db_pool: Pool,
        price_list: PriceList,
        start_date: NaiveDate,
        end_date: NaiveDate,
        warehouses_id: Vec<(i32, i32)>,
    ) -> Vec<(SettlementWarehouseReport, (Decimal, iso::Currency))> {
        warehouses_id
            .into_iter()
            .map(|(warehouse_id, warehouse_trade_partner_id)| {
                let size_reports = SettlementReport::gen_sizes(
                    db_pool.clone(),
                    price_list.clone(),
                    start_date,
                    end_date,
                    warehouse_id,
                );

                let size_reports_vec = size_reports
                    .clone()
                    .into_iter()
                    .map(|(report, _)| report)
                    .collect::<Vec<SettlementSizeReport>>();

                let price_sum: Decimal = size_reports
                    .clone()
                    .into_iter()
                    .map(|(_, price)| price.0.clone())
                    .sum();

                let currency = size_reports
                    .into_iter()
                    .map(|(_, price)| price.1)
                    .next()
                    .unwrap_or(*iso::PLN);

                let parcels_num = size_reports_vec
                    .clone()
                    .into_iter()
                    .map(|report| report.parcels_num)
                    .sum();
                (
                    SettlementWarehouseReport {
                        warehouse_id: warehouse_trade_partner_id,
                        sizes: size_reports_vec,
                        price: MoneyBody {
                            price: price_sum.to_string(),
                            currency: currency.to_string(),
                        },
                        parcels_num,
                    },
                    (price_sum, currency),
                )
            })
            .collect()
    }

    fn gen_sizes(
        db_pool: Pool,
        price_list: PriceList,
        start_date: NaiveDate,
        end_date: NaiveDate,
        warehouse_id: i32,
    ) -> Vec<(SettlementSizeReport, (Decimal, iso::Currency))> {
        ParcelSize::iterator()
            .filter_map(|size| {
                let price_list = price_list.clone();
                let price_opt = price_list.get(size);
                if price_opt.is_none() {
                    return None;
                }
                let price = price_opt.unwrap();

                let parcels = Parcel::find_by_date_and_warehouse_id(
                    db_pool.clone(),
                    start_date,
                    end_date,
                    warehouse_id,
                    size,
                );
                let parcels_num = parcels.len();
                let parcel_num_dec: Decimal = parcels_num.into();

                let price_sum = parcel_num_dec * price.amount().clone();

                Some((
                    SettlementSizeReport {
                        size,
                        unit_price: MoneyBody {
                            price: price.amount().to_string(),
                            currency: price.currency().to_string(),
                        },
                        parcels_id: parcels.into_iter().map(|parcel| parcel.id).collect(),
                        parcels_num,
                        price: MoneyBody {
                            price: price_sum.to_string(),
                            currency: price.currency().to_string(),
                        },
                    },
                    (price_sum.clone(), price.currency().clone()),
                ))
            })
            .collect()
    }
}
