use chrono::NaiveDate;

/// A common trait for anything that has a value on a given date.
pub trait FinancialInstrument {
    /// Computes the value of the instrument on the given date.
    fn value_on(&self, date: NaiveDate) -> f32;

    /// Returns the acquisition date (or start date) of the instrument.
    fn acquisition_date(&self) -> NaiveDate;

    fn name(&self) -> &str;
}
