//! [RFC9110: Date/Time Formats](https://www.rfc-editor.org/rfc/rfc9110#section-5.6.7)
//!
//! Obsolete formats:
//! - rfc850-date
//! - asctime-date

pub const GMT: &str = "GMT";

#[derive(Debug, PartialEq)]
pub struct HttpDate {
    ///  IMF-fixdate  = day-name "," SP date1 SP time-of-day SP GMT
    ///  ; fixed length/zone/capitalization subset of the format
    ///  ; see Section 3.3 of [RFC5322]

    /// "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"
    pub day_name: String,

    /// day SP month SP year
    /// e.g., 02 Jun 1982
    pub date: String,

    /// 2DIGIT
    pub day: u8,

    /// "Jan", "Feb", "Mar", "Apr"
    /// "May", "Jun", "Jul", "Aug"
    /// "Sep", "Oct", "Nov", "Dec"
    pub month: String,

    /// 4DIGIT
    pub year: u16,

    /// hour ":" minute ":" second
    /// 00:00:00 - 23:59:60  (leap second)
    pub time_of_day: String, 

    /// 2DIGIT
    pub hour: u8,
    /// 2DIGIT
    pub minute: u8,
    /// 2DIGIT
    pub second: u8,
}

impl HttpDate {
    pub fn from_str(v: &str) -> Option<HttpDate> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_imf_fixdate() {
        // Sun, 06 Nov 1994 08:49:37 GMT    ; IMF-fixdate
        let http_date = HttpDate::from_str("Sun, 06 Nov 1994 08:49:37 GMT").unwrap();
        assert_eq!(
            http_date,
            HttpDate {
                day_name: "Sun".to_string(),
                date: "06 Nov 1994".to_string(),
                day: 6,
                month: "Nov".to_string(),
                year: 1994,
                time_of_day: "08:49:37".to_string(),
                hour: 8,
                minute: 49,
                second: 37
            }
        );
    }

    #[test]
    fn from_str_obsolete_rfc850() {
        // Sunday, 06-Nov-94 08:49:37 GMT   ; obsolete RFC 850 format
        let http_date = HttpDate::from_str("Sunday, 06-Nov-94 08:49:37 GMT").unwrap();
        assert_eq!(
            http_date,
            HttpDate {
                day_name: "Sun".to_string(),
                date: "06 Nov 1994".to_string(),
                day: 6,
                month: "Nov".to_string(),
                year: 1994,
                time_of_day: "08:49:37".to_string(),
                hour: 8,
                minute: 49,
                second: 37
            }
        );
    }

    #[test]
    fn from_str_obsolete_asctime() {
        //  Sun Nov  6 08:49:37 1994         ; ANSI C's asctime() format
        let http_date = HttpDate::from_str("Sun Nov  6 08:49:37 1994").unwrap();
        assert_eq!(
            http_date,
            HttpDate {
                day_name: "Sun".to_string(),
                date: "06 Nov 1994".to_string(),
                day: 6,
                month: "Nov".to_string(),
                year: 1994,
                time_of_day: "08:49:37".to_string(),
                hour: 8,
                minute: 49,
                second: 37
            }
        );
    }
}
