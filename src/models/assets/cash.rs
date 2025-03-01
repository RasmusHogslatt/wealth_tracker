pub use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

use crate::{asset::AssetTrait, plot_utils::get_random_bytes_from_uuid};

use super::tradable::ContributionFrequency;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cash {
    pub uuid: Uuid,
    pub name: String,
    pub value: f32, // initial value
    pub acquisition_date: NaiveDate,
    pub contribution: f32, // amount added at each interval
    pub contribution_frequency: ContributionFrequency,
    pub should_delete: bool,
    pub color: egui::Color32,
}

impl Default for Cash {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        // create a red nuanced color from uuid
        let color = egui::Color32::from_rgb(
            70,
            get_random_bytes_from_uuid(&uuid),
            get_random_bytes_from_uuid(&uuid),
        );
        Self {
            uuid,
            name: "Cash".to_owned(),
            value: 1.0,
            acquisition_date: chrono::Utc::now().date_naive(),
            contribution: 1.0, // default contribution
            contribution_frequency: ContributionFrequency::Monthly, // default frequency
            should_delete: false,
            color,
        }
    }
}

impl AssetTrait for Cash {
    fn value(&self, date: NaiveDate) -> f32 {
        if date <= self.acquisition_date {
            return self.value;
        }

        let duration = date.signed_duration_since(self.acquisition_date);
        let total_days = duration.num_days() as f32;

        let interval_days = match self.contribution_frequency {
            ContributionFrequency::Weekly => 7.0,
            ContributionFrequency::Monthly => 30.0,
            ContributionFrequency::Yearly => 365.0,
        };

        let num_intervals = (total_days / interval_days).floor() as i32;
        let mut current_value = self.value;

        for _ in 0..num_intervals {
            current_value = (current_value + self.contribution).max(0.0);
        }

        current_value
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
