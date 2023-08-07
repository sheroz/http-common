//! [RFC9110: Date/Time Formats](https://www.rfc-editor.org/rfc/rfc9110#section-5.6.7)
//! 
//! Obsolete formats:
//! - rfc850-date
//! - asctime-date

pub struct HttpDate {
    pub day_name: String,
    pub date: String,
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub time_of_day: String,
    pub hour: u8,
    pub minute: u8,
    pub second: u8
}
