//! [RFC9110: Last-Modified](https://www.rfc-editor.org/rfc/rfc9110#section-8.8.2)

use crate::http_date::HttpDate;

pub struct HttpLastModified {
    pub last_modified: HttpDate
}