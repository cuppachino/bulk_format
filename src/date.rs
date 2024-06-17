use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let year_cmp = self.year.cmp(&other.year);
        let month_cmp = self.month.cmp(&other.month);
        let day_cmp = self.day.cmp(&other.day);
        year_cmp.then(month_cmp).then(day_cmp)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp_dates() {
        let date1 = Date { year: 2020, month: Some(1), day: Some(1) };
        let date2 = Date { year: 2020, month: Some(1), day: Some(2) };
        let date3 = Date { year: 2020, month: Some(2), day: Some(1) };
        let date4 = Date { year: 2021, month: Some(1), day: Some(1) };
        assert!(date1 < date2);
        assert!(date2 < date3);
        assert!(date3 < date4);
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Date { year, month: None, day: None } => write!(f, "{}", year),
            Date { year, month: Some(month), day: None } => write!(f, "{}-{}", year, month),
            Date { year, month: Some(month), day: Some(day) } =>
                write!(f, "{}-{}-{}", year, month, day),
            _ => panic!("Invalid date. Has a year and day, but no month"),
        }
    }
}

impl TryFrom<&str> for Date {
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let parts = string.split('-').collect::<Vec<&str>>();
        match parts.len() {
            1 =>
                Ok(Date {
                    year: parts[0].parse().map_err(|_| "Invalid year")?,
                    month: None,
                    day: None,
                }),
            2 => {
                Ok(Date {
                    year: parts[0].parse().map_err(|_| "Invalid year")?,
                    month: Some(parts[1].parse().map_err(|_| "Invalid month")?),
                    day: None,
                })
            }
            3 => {
                Ok(Date {
                    year: parts[0].parse().map_err(|_| "Invalid year")?,
                    month: Some(parts[1].parse().map_err(|_| "Invalid month")?),
                    day: Some(parts[2].parse().map_err(|_| "Invalid day")?),
                })
            }
            _ => Err("Invalid date".to_string()),
        }
    }
}
