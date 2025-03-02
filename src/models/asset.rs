use super::{Cash, Loan, RealEstate, Tradable};
use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

pub trait AssetTrait {
    fn value(&self, date: NaiveDate) -> f32;
    fn name(&self) -> String;
    fn ui_edit(&mut self, ui: &mut Ui, currency: String) -> bool;
    fn uuid(&self) -> Uuid;
    fn should_delete(&self) -> bool {
        false
    }
    fn color(&self) -> egui::Color32;
    fn is_growth(&self) -> bool;
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum Asset {
    RealEstate(RealEstate),
    Loan(Loan),
    Tradable(Tradable),
    Cash(Cash),
}

impl AssetTrait for Asset {
    fn value(&self, date: NaiveDate) -> f32 {
        match self {
            Asset::RealEstate(real_estate) => real_estate.value(date),
            Asset::Loan(loan) => loan.value(date),
            Asset::Tradable(tradable) => tradable.value(date),
            Asset::Cash(cash) => cash.value(date),
        }
    }

    fn name(&self) -> String {
        match self {
            Asset::RealEstate(real_estate) => real_estate.name.clone(),
            Asset::Loan(loan) => loan.name.clone(),
            Asset::Tradable(tradable) => tradable.name.clone(),
            Asset::Cash(cash) => cash.name.clone(),
        }
    }
    fn ui_edit(&mut self, ui: &mut Ui, currency: String) -> bool {
        match self {
            Asset::RealEstate(real_estate) => real_estate.ui_edit(ui, currency),
            Asset::Loan(loan) => loan.ui_edit(ui, currency),
            Asset::Tradable(tradable) => tradable.ui_edit(ui, currency),
            Asset::Cash(cash) => cash.ui_edit(ui, currency),
        }
    }
    fn uuid(&self) -> Uuid {
        match self {
            Asset::RealEstate(real_estate) => real_estate.uuid,
            Asset::Loan(loan) => loan.uuid,
            Asset::Tradable(tradable) => tradable.uuid,
            Asset::Cash(cash) => cash.uuid,
        }
    }
    fn should_delete(&self) -> bool {
        match self {
            Asset::RealEstate(real_estate) => real_estate.should_delete(),
            Asset::Loan(loan) => loan.should_delete(),
            Asset::Tradable(tradable) => tradable.should_delete(),
            Asset::Cash(cash) => cash.should_delete(),
        }
    }
    fn color(&self) -> egui::Color32 {
        match self {
            Asset::RealEstate(real_estate) => real_estate.color(),
            Asset::Loan(loan) => loan.color(),
            Asset::Tradable(tradable) => tradable.color(),
            Asset::Cash(cash) => cash.color(),
        }
    }
    fn is_growth(&self) -> bool {
        match self {
            Asset::RealEstate(real_estate) => real_estate.is_growth(),
            Asset::Loan(loan) => loan.is_growth(),
            Asset::Tradable(tradable) => tradable.is_growth(),
            Asset::Cash(cash) => cash.is_growth(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum AssetType {
    RealEstate,
    Loan,
    Tradable,
    Cash,
}
