pub use chrono::NaiveDate;

use crate::asset::AssetTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct Loan {
    pub name: String,
    pub value: f32,
    pub rate_per_year: f32,
    pub acquisition_date: NaiveDate,
    pub monthly_principal: f32,
}
impl Default for Loan {
    fn default() -> Self {
        Self {
            name: "House loan".to_owned(),
            value: 500000.0,
            rate_per_year: 7.0, // 5% value growth per year
            acquisition_date: chrono::Utc::now().date_naive(),
            monthly_principal: 3000.0,
        }
    }
}

impl AssetTrait for Loan {
    fn value(&self, date: NaiveDate) -> f32 {
        // If the date is on or before the loan's acquisition, return the initial principal.
        if date <= self.acquisition_date {
            return self.value;
        }

        // Determine the total elapsed time in days since acquisition.
        let duration = date.signed_duration_since(self.acquisition_date);
        let total_days = duration.num_days();

        // Approximate each month as 30 days.
        let full_months = total_days / 30;
        let remaining_days = total_days % 30;

        // Compute the total principal paid over all full months.
        let principal_paid = self.monthly_principal * (full_months as f32);

        // Ensure the remaining principal never goes below zero.
        let mut remaining_principal = if self.value > principal_paid {
            self.value - principal_paid
        } else {
            0.0
        };

        // For any partial month, compute the accrued interest on the remaining principal.
        // Since interest is paid monthly, it is not compounded into the principal.
        if remaining_principal > 0.0 && remaining_days > 0 {
            let fraction_of_month = remaining_days as f32 / 30.0;
            // Derive the monthly interest rate from the annual rate.
            let monthly_rate = self.rate_per_year / 100.0 / 12.0;
            // Accrued interest for the partial month:
            let accrued_interest = remaining_principal * monthly_rate * fraction_of_month;
            remaining_principal += accrued_interest;
        }

        remaining_principal
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
