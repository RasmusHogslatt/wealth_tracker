pub use chrono::NaiveDate;
use egui::Ui;
use uuid::Uuid;

use crate::{asset::AssetTrait, plot_utils::get_random_bytes_from_uuid};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RealEstate {
    pub uuid: Uuid,
    pub name: String,
    pub value: f32,
    pub rate_per_year: f32,
    pub acquisition_date: NaiveDate,
    pub should_delete: bool,
    pub color: egui::Color32,
}
impl Default for RealEstate {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        // create red nuanced color from uuid
        let color = egui::Color32::from_rgb(70, get_random_bytes_from_uuid(&uuid), 70);
        Self {
            uuid,
            name: "Real Estate".to_owned(),
            value: 110000.0,
            rate_per_year: 5.0,
            acquisition_date: chrono::Utc::now().date_naive(),
            should_delete: false,
            color,
        }
    }
}

impl AssetTrait for RealEstate {
    fn value(&self, date: NaiveDate) -> f32 {
        // If the provided date is before (or on) the acquisition date,
        // we return the initial value.
        if date <= self.acquisition_date {
            return self.value;
        }

        // Calculate the number of days elapsed since the acquisition.
        let duration = date.signed_duration_since(self.acquisition_date);
        let days_elapsed = duration.num_days() as f32;

        // Convert days elapsed into fractional years.
        let years_elapsed = days_elapsed / 365.0; // You can adjust to 365.25 if needed.

        // Compute the growth factor. Here, rate_per_year is assumed to be in percent.
        let growth_multiplier = (1.0 + self.rate_per_year / 100.0).powf(years_elapsed);

        // The asset's value on the given date is the initial value multiplied by the growth factor.
        self.value * growth_multiplier
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
                            .prefix(currency),
                    )
                    .changed();
            });

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

            ui.horizontal(|ui| {
                ui.label("Acquisition Date: ");
                // You might want to add a date picker here
                // For now, we'll just show the date
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
