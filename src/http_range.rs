//! Implemented according
//!  * WIP [RFC7233](https://datatracker.ietf.org/doc/html/rfc7233)
//!  * WIP [RFC7232](https://datatracker.ietf.org/doc/html/rfc7232)
//!  * WIP [RFC2616](https://www.ietf.org/rfc/rfc2616)
//! 
//! Additional resources:
//!
//! https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests
//!
//! https://http.dev/range-request

use std::ops::Range;
pub static RANGE_UNIT: &str = "bytes";

#[derive(Debug, PartialEq)]
pub struct HttpRange {
    pub ranges: Vec<Range<u64>>,
    pub complete_length: Option<CompleteLength>,
}

#[derive(Debug, PartialEq)]
pub enum CompleteLength {
    Representation(u64),
    Unknown,
}

pub fn parse(range_value: &str, bytes_count: u64) -> Option<HttpRange> {
    if range_value.is_empty() {
        return None;
    }

    let parts = range_value.split("=").map(|p| p.trim()).collect::<Vec<_>>();
    if parts.is_empty() {
        return None;
    }

    if parts[0] != RANGE_UNIT {
        return None;
    }

    if parts.len() != 2 {
        return None;
    }

    let params = parts[1].split("/").map(|p| p.trim()).collect::<Vec<_>>();
    if params.is_empty() {
        return None;
    }

    if params.len() > 2 {
        return None;
    }

    let range_params = params[0].split(",");
    let length_param = if params.len() == 2 { params[1] } else { "" };

    let mut ranges = Vec::<Range<u64>>::new();
    for range_param in range_params {
        let values = range_param.split("-").map(|v| v.trim()).collect::<Vec<_>>();
        if values.len() != 2 {
            return None;
        }
        let mut range = 0..bytes_count - 1;
        let start = values[0];
        let end = values[1];

        if !start.is_empty() && !end.is_empty() {
            range.start = start.parse::<u64>().unwrap();
            range.end = end.parse::<u64>().unwrap();
        }
        if start.is_empty() && !end.is_empty() {
            let count = end.parse::<u64>().unwrap();
            range.start = bytes_count - count;
        }

        if !start.is_empty() && end.is_empty() {
            range.start = start.parse::<u64>().unwrap();
        }

        ranges.push(range);
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let ranges_count = ranges.len();
    if ranges_count > 1 {
        // merge continuous and overlapping ranges
        let mut retain = vec![true; ranges_count];
        let mut range_last = ranges[0].clone();
        for (index, range) in ranges.iter_mut().enumerate() {
            if index != 0 && (range_last.end + 1) >= range.start {
                range.start = range_last.start;
                retain[index - 1] = false;
            }
            range_last = range.clone();
        }

        // clean-up merged ranges
        let mut index = 0;
        ranges.retain(|_| {
            let keep = retain[index];
            index += 1;
            keep
        });
    }

    let complete_length = match length_param {
        "" => None,
        "*" => Some(CompleteLength::Unknown),
        _ => Some(CompleteLength::Representation(length_param.parse::<u64>().unwrap()))
    };

    let http_range = HttpRange {
        ranges,
        complete_length,
    };

    Some(http_range)
}

pub fn none_satisfiable(http_range: &HttpRange, content_length: u64 ) -> bool {
    !any_satisfiable(http_range, content_length)
}

pub fn any_satisfiable(http_range: &HttpRange, content_length: u64 ) -> bool {
    // 416 Range Not Satisfiable
    // https://datatracker.ietf.org/doc/html/rfc7233#section-4.4
    for range in &http_range.ranges {
        if range_satisfiable(range, content_length) {
            return true;
        }
    }
    false
}

pub fn range_satisfiable(range: &Range<u64>, content_length: u64 ) -> bool {
    // 416 Range Not Satisfiable
    // https://datatracker.ietf.org/doc/html/rfc7233#section-4.4
    range.start < content_length
}

#[cfg(test)]
mod tests {
    use super::*;
    /// https://datatracker.ietf.org/doc/html/rfc7233#section-4.2
    ///
    /// Examples of byte-ranges-specifier values:
    ///    -  The first 500 bytes (byte offsets 0-499, inclusive):
    ///         bytes=0-499
    ///    -  The second 500 bytes (byte offsets 500-999, inclusive):
    ///         bytes=500-999
    ///
    /// Additional examples, assuming a representation of length 10000:
    ///    The final 500 bytes (byte offsets 9500-9999, inclusive):
    ///         bytes=-500
    ///    Or:
    ///         bytes=9500-
    ///    -  The first and last bytes only (bytes 0 and 9999):
    ///         bytes=0-0,-1
    ///    -  Other valid (but not canonical) specifications of the second 500
    ///       bytes (byte offsets 500-999, inclusive):
    ///         bytes=500-600,601-999
    ///         bytes=500-700,601-999
    /// 
    /// Additional examples:
    ///    https://datatracker.ietf.org/doc/html/rfc7233#section-4.2
    /// 
    ///     - The first 500 bytes:
    ///         Content-Range: bytes 0-499/1234
    /// 
    ///     - The second 500 bytes:
    ///         Content-Range: bytes 500-999/1234
    /// 
    ///     - All except for the first 500 bytes:
    ///         Content-Range: bytes 500-1233/1234
    ///  
    ///     - The last 500 bytes:
    ///         Content-Range: bytes 734-1233/1234

    #[test]
    fn test1() {
        let http_range = parse("bytes=0-499", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![0..499],
                complete_length: None
            }
        );
    }

    #[test]
    fn complete_length_unknown() {
        let http_range = parse("bytes=0-499/*", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![0..499],
                complete_length: Some(CompleteLength::Unknown)
            }
        );
    }

    #[test]
    fn complete_length_test2() {
        let http_range = parse("bytes=0-499/8000", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![0..499],
                complete_length: Some(CompleteLength::Representation(8000))
            }
        );
    }

    #[test]
    fn test2() {
        let http_range = parse("bytes=500-999", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..999],
                complete_length: None
            }
        );
    }

    #[test]
    fn test3() {
        let http_range = parse("bytes=-500", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![9500..9999],
                complete_length: None
            }
        );
    }

    #[test]
    fn test4() {
        let http_range = parse("bytes=9500-", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![9500..9999],
                complete_length: None
            }
        );
    }

    #[test]
    fn test5() {
        let http_range = parse("bytes=0-0,-1", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![0..0, 9999..9999],
                complete_length: None
            }
        );
    }

    #[test]
    fn combined_merge_test6() {
        let http_range = parse("bytes=500-600,601-999", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..999],
                complete_length: None
            }
        );
    }

    #[test]
    fn combined_merge_test7() {
        let http_range = parse("bytes=601-999,500-600", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..999],
                complete_length: None
            }
        );
    }

    #[test]
    fn combined_merge_test8() {
        let http_range = parse("bytes=500-700,601-999", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..999],
                complete_length: None
            }
        );
    }

    #[test]
    fn combined_merge_test9() {
        let http_range = parse("bytes=601-999,500-700", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..999],
                complete_length: None
            }
        );
    }

    #[test]
    fn combined_merge_test10() {
        let http_range = parse("bytes=300-400,400-700,601-999", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![300..999],
                complete_length: None
            }
        );
    }

    #[test]
    fn representation_test1() {
        let http_range = parse("bytes=0-499/1234", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![0..499],
                complete_length: Some(CompleteLength::Representation(1234))
            }
        );
    }

    #[test]
    fn representation_test2() {
        let http_range = parse("bytes=500-999/1234", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..999],
                complete_length: Some(CompleteLength::Representation(1234))
            }
        );
    }

    #[test]
    fn representation_test3() {
        let http_range = parse("bytes=500-1233/1234", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![500..1233],
                complete_length: Some(CompleteLength::Representation(1234))
            }
        );
    }

    #[test]
    fn representation_test4() {
        let http_range = parse("bytes=734-1233/1234", 10000).unwrap();
        assert_eq!(
            http_range,
            HttpRange {
                ranges: vec![734..1233],
                complete_length: Some(CompleteLength::Representation(1234))
            }
        );
    }
}
