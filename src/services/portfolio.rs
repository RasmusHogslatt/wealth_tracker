use crate::models::FinancialInstrument;
use chrono::NaiveDate;

/// A portfolio that can hold various financial instruments.
pub struct Portfolio {
    /// You might have separate vectors for assets, liabilities, investments,
    /// or combine them into one if they share the same behavior.
    pub instruments: Vec<Box<dyn FinancialInstrument>>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self {
            instruments: Vec::new(),
        }
    }

    /// Add a new instrument to the portfolio.
    pub fn add_instrument(&mut self, instrument: Box<dyn FinancialInstrument>) {
        self.instruments.push(instrument);
    }

    /// Computes the total portfolio value on a given date.
    pub fn total_value_on(&self, date: NaiveDate) -> f32 {
        self.instruments
            .iter()
            .map(|instr| instr.value_on(date))
            .sum()
    }

    /// Generate a series of (date, total_value) tuples between start and end dates,
    /// using the given interval (in days).
    pub fn value_over_time(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        interval_days: i64,
    ) -> Vec<(NaiveDate, f32)> {
        let mut values = Vec::new();
        let mut current = start;
        while current <= end {
            let value = self.total_value_on(current);
            values.push((current, value));
            current =
                current.succ_opt().unwrap_or(current) + chrono::Duration::days(interval_days - 1);
            // Alternatively, use a loop counter and add Duration::days(interval_days).
        }
        values
    }
}
