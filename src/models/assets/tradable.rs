pub use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

use crate::{asset::AssetTrait, plot_utils::get_random_bytes_from_uuid};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ContributionFrequency {
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Tradable {
    pub uuid: Uuid,
    pub name: String,
    pub value: f32,         // initial value
    pub rate_per_year: f32, // annual growth rate (%)
    pub acquisition_date: NaiveDate,
    pub contribution: f32, // amount added at each interval
    pub contribution_frequency: ContributionFrequency,
    pub should_delete: bool,
    pub color: egui::Color32,
}

impl Default for Tradable {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        // create a red nuanced color from uuid
        let color = egui::Color32::from_rgb(0, get_random_bytes_from_uuid(&uuid), 0);
        Self {
            uuid,
            name: "Stocks".to_owned(),
            value: 1.0,
            rate_per_year: 8.0,
            acquisition_date: chrono::Utc::now().date_naive(),
            contribution: 1.0, // default contribution
            contribution_frequency: ContributionFrequency::Monthly, // default frequency
            should_delete: false,
            color,
        }
    }
}

impl AssetTrait for Tradable {
    fn value(&self, date: NaiveDate) -> f32 {
        // If the target date is on or before the acquisition, return the initial value.
        if date <= self.acquisition_date {
            return self.value;
        }

        let duration = date.signed_duration_since(self.acquisition_date);
        let total_days = duration.num_days() as f32;
        let years_elapsed = total_days / 365.0;
        let rate = self.rate_per_year / 100.0;

        // Compound the initial investment.
        let compounded_initial = self.value * (1.0 + rate).powf(years_elapsed);

        // Determine the interval (in days) based on the contribution frequency.
        let interval_days = match self.contribution_frequency {
            ContributionFrequency::Weekly => 7.0,
            ContributionFrequency::Monthly => 30.0,
            ContributionFrequency::Yearly => 365.0,
        };

        // Determine how many contribution periods have passed.
        let num_periods = (total_days / interval_days).floor() as i32;
        let mut compounded_contributions = 0.0;

        // For each contribution, compound it from its deposit date until the target date.
        for i in 1..=num_periods {
            // Compute the contribution date.
            let contribution_date =
                self.acquisition_date + chrono::Duration::days((i as f32 * interval_days) as i64);
            if contribution_date > date {
                break;
            }
            let days_since_contribution = (date - contribution_date).num_days() as f32;
            let years_since_contribution = days_since_contribution / 365.0;
            compounded_contributions +=
                self.contribution * (1.0 + rate).powf(years_since_contribution);
        }

        compounded_initial + compounded_contributions
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn ui_edit(&mut self, ui: &mut Ui) -> bool {
        let mut modified = false;

        ui.group(|ui| {
            // Name and Delete button.
            ui.horizontal(|ui| {
                modified |= ui.text_edit_singleline(&mut self.name).changed();
                if ui.small_button("Delete").clicked() {
                    self.should_delete = true;
                }
            });

            // Initial Value.
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

            // Annual Growth Rate.
            ui.horizontal(|ui| {
                ui.label("Annual Rate (%): ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.rate_per_year)
                            .speed(0.1)
                            .range(-20.0..=20.0),
                    )
                    .changed();
            });

            // Contribution Amount.
            ui.horizontal(|ui| {
                ui.label("Contribution: ");
                modified |= ui
                    .add(
                        egui::DragValue::new(&mut self.contribution)
                            .speed(10.0)
                            .prefix("$"),
                    )
                    .changed();
            });

            // Contribution Frequency.
            ui.horizontal(|ui| {
                ui.label("Frequency: ");
                egui::ComboBox::from_id_salt("tradable_frequency")
                    .selected_text(format!("{:?}", self.contribution_frequency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.contribution_frequency,
                            ContributionFrequency::Weekly,
                            "Weekly",
                        );
                        ui.selectable_value(
                            &mut self.contribution_frequency,
                            ContributionFrequency::Monthly,
                            "Monthly",
                        );
                        ui.selectable_value(
                            &mut self.contribution_frequency,
                            ContributionFrequency::Yearly,
                            "Yearly",
                        );
                    });
            });

            // Acquisition Date (display only for now).
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
        true
    }
}
