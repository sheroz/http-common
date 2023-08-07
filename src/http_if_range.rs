//! [RFC9110: If-Range](https://www.rfc-editor.org/rfc/rfc9110.html#section-13.1.5)

use crate::http_date::HttpDate;

pub enum HttpIfRange {
    HttpDate(HttpDate),
    HttpETag(String)
}

