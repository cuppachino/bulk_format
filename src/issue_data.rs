use owo_colors::OwoColorize;
use crate::{ print_warn, print_warn_ok };

#[derive(Debug)]
pub struct IssueData {
    pub tn: String,
    pub title: String,
    pub volume: Option<u32>,
    pub issue: Option<u32>,
    pub date: String,
    /// A date string in the format: `d/m/y h:m`.
    pub date_loaded: String,
}

impl IssueData {
    pub fn new(tn: String, raw_title: String, date_loaded: String) -> Self {
        // Example: Arizona Catering Employees. (Aug. 6, 1944)
        // Example: Arizona Catering Employees. v. 1 no 11 Sep. 21, 1944)
        // Example: Arizona Catering Employees. v. 9, no. 9 (Jul. 11, 1952)

        // First, split off the date.
        let mut parts = raw_title.split(" (");
        let date = parts
            .clone()
            .last()
            .expect("Failed to split date from raw title")
            .trim_end_matches(')');
        let date = convert_date(date);

        // Next, split off the volume and issue, if they exist.
        let parts = parts.next().expect("Failed to split title from raw title");
        let (title, v_n) = parts
            .split_once(".")
            .expect(
                format!("Failed to split title from volume/issue for title: \"{}\"", parts).as_str()
            );

        // Replace spaces in the title with underscores.
        let title = title.replace(" ", "_");

        // If the volume and issue are not empty, split them.
        let (volume, issue) = if v_n.is_empty() {
            (None, None)
        } else {
            let (volume, issue) = match v_n.split_once(", ") {
                Some((v, n)) => (v.to_string(), n.to_string()),
                None => {
                    let v_n = v_n
                        .trim()
                        .replace("no. ", "no.")
                        .replace("no ", "no")
                        .replace("v. ", "v.")
                        .replace("v ", "v");

                    print_warn!("VOLUME AND ISSUE WERE NOT COMMA SEPARATED: \"{}\"", v_n);

                    let (v, n) = v_n.split_once(" ").expect("Failed to split volume/issue.");
                    print_warn_ok!(
                        "Split volume/issue on the first space instead: \"{}\" and \"{}\"",
                        v,
                        n
                    );
                    (v.to_string(), n.to_string())
                }
            };

            let volume = volume
                .trim()
                .trim_start_matches("v. ")
                .trim_start_matches("v.")
                .trim_start_matches("v ")
                .trim_start_matches("v")
                .trim_end_matches(|c: char| (c == ',' || c == '.' || c.is_whitespace()));

            let issue = issue
                .trim()
                .trim_start_matches("no. ")
                .trim_start_matches("no.")
                .trim_start_matches("no ")
                .trim_start_matches("no")
                .trim_end_matches(|c: char| (c == ',' || c == '.' || c.is_whitespace()));

            let volume = volume
                .parse::<u32>()
                .expect(format!("Failed to parse volume: {}", volume).as_str());

            let issue = (
                match issue.parse::<u32>() {
                    Ok(i) => Some(i),
                    Err(_) => {
                        // The date is probably missing its opening parenthesis and is being parsed as the issue. Try splitting on the first space, and taking the first part.
                        issue.split_once(" ").expect("Failed to split issue.").0.parse::<u32>().ok()
                    }
                }
            ).expect("Failed to parse issue.");

            (Some(volume), Some(issue))
        };

        Self {
            tn,
            title,
            volume,
            issue,
            date,
            date_loaded,
        }
    }

    /// Returns a formatted title for the issue in the format: `title_date`.
    pub fn formatted_title(&self) -> String {
        format!("{}_{}", self.title, self.date)
    }

    /// Returns a formatted title for the issue in the format: `title, date`.
    pub fn record_title(&self) -> String {
        // replace underscores with spaces.
        let title = self.title.replace("_", " ");
        format!("{}, {}", title, self.date)
    }
}

const MONTHS: [&str; 12] = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];

/// Converts a date string to a partial yyyy-mm-dd format.
///
/// Example: Aug. 6, 1944 -> 1944-08-06
fn convert_date(date: &str) -> String {
    let mut parts = date.split(" ").collect::<Vec<&str>>();

    if parts.len() != 3 {
        print_warn!("Parts is not 3 in length. Date: \"{}\"", date);
        let mut month_i = None;
        for (i, part) in parts.iter().enumerate() {
            if month_i.is_some() {
                break;
            }
            for month in MONTHS.iter() {
                if part.starts_with(month) {
                    print_warn_ok!("Found month: \"{}\" at part {}", month, i);
                    month_i = Some(i);
                    break;
                }
            }
        }
        let month_i = month_i.expect(
            format!("Failed to find month in date. Check the date formatting for: \"{}\"", date).as_str()
        );
        // replace parts with month_i..end.
        parts = parts[month_i..].to_vec();
    }

    // assert_eq!(parts.len(), 3);

    match parts.len() {
        3 => {
            let month = parts[0].trim_end_matches('.');
            let day = parts[1].trim_end_matches(',');
            let year = parts[2];

            let month = try_parse_month(month);
            let day = if day.len() == 1 { format!("0{}", day) } else { day.to_string() };

            format!("{}-{}-{}", year, month, day)
        }
        2 => {
            let month = parts[0].trim_end_matches('.');
            let year = parts[1];

            let month = try_parse_month(month);
            format!("{}-{}", year, month)
        }
        1 => {
            let year = parts[0];
            format!("{}", year)
        }
        _ => panic!("Invalid date."),
    }
}

fn try_parse_month(maybe_month: &str) -> &str {
    match maybe_month {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => panic!("Invalid month."),
    }
}
