pub use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

use crate::{asset::AssetTrait, plot_utils::get_random_bytes_from_uuid};

use super::tradable::ContributionFrequency;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Loan {
    pub uuid: Uuid,
    pub name: String,
    pub value: f32,
    pub rate_per_year: f32,
    pub acquisition_date: NaiveDate,
    pub principal_payment: f32,
    pub principal_frequency: ContributionFrequency, // New field
    pub should_delete: bool,
    pub color: egui::Color32,
}
impl Default for Loan {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        let color = egui::Color32::from_rgb(get_random_bytes_from_uuid(&uuid), 70, 70);
        Self {
            uuid,
            name: "New Loan".to_owned(),
            value: 100000.0,
            rate_per_year: 5.0,
            acquisition_date: chrono::Utc::now().date_naive(),
            should_delete: false,
            color,
            principal_payment: 0.0,
            principal_frequency: ContributionFrequency::Monthly,
        }
    }
}

impl AssetTrait for Loan {
    fn value(&self, date: NaiveDate) -> f32 {
        if date <= self.acquisition_date {
            return self.value;
        }

        let duration = date.signed_duration_since(self.acquisition_date);
        let total_days = duration.num_days() as f32;

        // Determine the interval length and corresponding rate per interval.
        let interval_days = match self.principal_frequency {
            ContributionFrequency::Weekly => 7.0,
            ContributionFrequency::Monthly => 30.0,
            ContributionFrequency::Yearly => 365.0,
        };

        let rate_per_interval =
            (1.0 + self.rate_per_year / 100.0).powf(interval_days / 365.0) - 1.0;
        let num_intervals = (total_days / interval_days).floor() as i32;
        let remaining_days = total_days % interval_days;

        // Start with the initial principal.
        let mut remaining_principal = self.value;

        // For each full interval:
        for _ in 0..num_intervals {
            remaining_principal += remaining_principal * rate_per_interval;
            remaining_principal = (remaining_principal - self.principal_payment).max(0.0);
        }

        // Handle any remaining partial interval.
        if remaining_days > 0.0 && remaining_principal > 0.0 {
            let fractional_rate =
                (1.0 + self.rate_per_year / 100.0).powf((remaining_days) / 365.0) - 1.0;
            remaining_principal += remaining_principal * fractional_rate;
        }

        remaining_principal
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn ui_edit(&mut self, ui: &mut Ui, currency: String) -> bool {
        let mut modified = false;

        ui.group(|ui| {
            ui.horizontal(|ui| {
                modified |= ui.text_edit_singleline(&mut self.name).changed();
            });

            ui.horizontal(|ui| {
                ui.label("Value: ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.value)
                            .speed(1000.0)
                            .prefix(currency.clone()),
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
                ui.label("Principal: ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.principal_payment)
                            .speed(100.0)
                            .prefix(currency),
                    )
                    .changed();
            });
            // Contribution Frequency.
            ui.horizontal(|ui| {
                ui.label("Frequency: ");
                egui::ComboBox::from_id_salt("loan_frequency")
                    .selected_text(format!("{:?}", self.principal_frequency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.principal_frequency,
                            ContributionFrequency::Weekly,
                            "Weekly",
                        );
                        ui.selectable_value(
                            &mut self.principal_frequency,
                            ContributionFrequency::Monthly,
                            "Monthly",
                        );
                        ui.selectable_value(
                            &mut self.principal_frequency,
                            ContributionFrequency::Yearly,
                            "Yearly",
                        );
                    });
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

    fn should_delete(&self) -> bool {
        self.should_delete
    }

    fn color(&self) -> egui::Color32 {
        self.color
    }

    fn is_growth(&self) -> bool {
        false
    }
}
