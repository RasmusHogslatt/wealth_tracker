use super::{Loan, RealEstate};
use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

pub trait AssetTrait {
    fn value(&self, date: NaiveDate) -> f32;
    fn name(&self) -> String;
    fn ui_edit(&mut self, ui: &mut Ui) -> bool;
    fn uuid(&self) -> Uuid;
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Asset {
    RealEstate(RealEstate),
    Loan(Loan),
}

impl AssetTrait for Asset {
    fn value(&self, date: NaiveDate) -> f32 {
        match self {
            Asset::RealEstate(real_estate) => real_estate.value(date),
            Asset::Loan(loan) => loan.value(date),
        }
    }

    fn name(&self) -> String {
        match self {
            Asset::RealEstate(real_estate) => real_estate.name.clone(),
            Asset::Loan(loan) => loan.name.clone(),
        }
    }
    fn ui_edit(&mut self, ui: &mut Ui) -> bool {
        match self {
            Asset::RealEstate(real_estate) => real_estate.ui_edit(ui),
            Asset::Loan(loan) => loan.ui_edit(ui),
        }
    }
    fn uuid(&self) -> Uuid {
        match self {
            Asset::RealEstate(real_estate) => real_estate.uuid,
            Asset::Loan(loan) => loan.uuid,
        }
    }
}

#[derive(PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum AssetType {
    RealEstate,
    Loan,
}
