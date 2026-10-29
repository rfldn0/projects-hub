//! Calendar dates as written in projects.json (`YYYY-MM-DD`), without a date library.

use std::fmt;

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

/// Field order makes the derived `Ord` chronological.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn parse(s: &str) -> Option<Date> {
        let mut parts = s.split('-');
        let (y, m, d) = (parts.next()?, parts.next()?, parts.next()?);
        let digits = |p: &str, len| p.len() == len && p.bytes().all(|b| b.is_ascii_digit());
        if parts.next().is_some() || !digits(y, 4) || !digits(m, 2) || !digits(d, 2) {
            return None;
        }
        let date = Date { year: y.parse().ok()?, month: m.parse().ok()?, day: d.parse().ok()? };
        ((1..=12).contains(&date.month) && (1..=31).contains(&date.day)).then_some(date)
    }

    fn abbr(&self) -> &'static str {
        &MONTHS[usize::from(self.month - 1)][..3]
    }

    /// "Oct 27"
    pub fn short(&self) -> String {
        format!("{} {:02}", self.abbr(), self.day)
    }

    /// "Oct 27, 2026"
    pub fn long(&self) -> String {
        format!("{} {}, {}", self.abbr(), self.day, self.year)
    }

    /// "October 2026"
    pub fn month_year(&self) -> String {
        format!("{} {}", MONTHS[usize::from(self.month - 1)], self.year)
    }
}

impl TryFrom<String> for Date {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Date::parse(&s).ok_or_else(|| format!("invalid date `{s}`, expected YYYY-MM-DD"))
    }
}

/// ISO form, used for `<time datetime>`.
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn parses_and_formats() {
        let d = Date::parse("2026-10-07").unwrap();
        assert_eq!(d.short(), "Oct 07");
        assert_eq!(d.long(), "Oct 7, 2026");
        assert_eq!(d.month_year(), "October 2026");
        assert_eq!(d.to_string(), "2026-10-07");
    }

    #[test]
    fn rejects_malformed() {
        for bad in ["2026-13-01", "2026-1-01", "26-10-01", "2026-10-01-1", "2026-10-00", "+026-10-01"] {
            assert!(Date::parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn orders_across_years() {
        assert!(Date::parse("2025-12-31") < Date::parse("2026-01-01"));
    }
}
