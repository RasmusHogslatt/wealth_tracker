use chrono::NaiveDate;

use super::{Loan, RealEstate};

pub trait AssetTrait {
    fn value(&self, date: NaiveDate) -> f32;
    fn name(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
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
}

#[derive(PartialEq, Clone, Debug)]
pub enum AssetType {
    RealEstate,
    Loan,
}
