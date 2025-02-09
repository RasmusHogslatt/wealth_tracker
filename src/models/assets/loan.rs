pub use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

use crate::asset::AssetTrait;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Loan {
    pub uuid: Uuid,
    pub name: String,
    pub value: f32,
    pub rate_per_year: f32,
    pub acquisition_date: NaiveDate,
    pub monthly_principal: f32,
}
impl Default for Loan {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: "New Loan".to_owned(),
            value: 100000.0,
            rate_per_year: 5.0,
            acquisition_date: chrono::Utc::now().date_naive(),
            monthly_principal: 0.0,
        }
    }
}

impl AssetTrait for Loan {
    fn value(&self, date: NaiveDate) -> f32 {
        if date <= self.acquisition_date {
            return self.value;
        }

        let duration = date.signed_duration_since(self.acquisition_date);
        let total_days = duration.num_days();
        let full_months = total_days / 30;
        let remaining_days = total_days % 30;

        // Start with initial principal
        let mut remaining_principal = self.value;
        let monthly_rate = self.rate_per_year / 100.0 / 12.0;

        // For each full month:
        // 1. Add interest for the month
        // 2. Subtract principal payment
        for _ in 0..full_months {
            remaining_principal += remaining_principal * monthly_rate;
            remaining_principal = (remaining_principal - self.monthly_principal).max(0.0);
        }

        // Handle any remaining partial month
        if remaining_principal > 0.0 && remaining_days > 0 {
            let fraction_of_month = remaining_days as f32 / 30.0;
            let accrued_interest = remaining_principal * monthly_rate * fraction_of_month;
            remaining_principal += accrued_interest;
        }

        remaining_principal
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn ui_edit(&mut self, ui: &mut Ui) -> bool {
        let mut modified = false;

        ui.group(|ui| {
            modified |= ui.text_edit_singleline(&mut self.name).changed();

            ui.horizontal(|ui| {
                ui.label("Value: ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.value)
                            .speed(1000.0)
                            .prefix("$"),
                    )
                    .changed();
            });

            ui.horizontal(|ui| {
                ui.label("Interest Rate (%): ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.rate_per_year)
                            .speed(0.1)
                            .range(0.0..=20.0),
                    )
                    .changed();
            });

            ui.horizontal(|ui| {
                ui.label("Monthly Principal: ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.monthly_principal)
                            .speed(100.0)
                            .prefix("$"),
                    )
                    .changed();
            });

            ui.horizontal(|ui| {
                ui.label("Acquisition Date: ");
                ui.label(self.acquisition_date.to_string());
            });
        });

        modified
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}
