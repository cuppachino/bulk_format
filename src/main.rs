use std::{ collections::BTreeMap, path::PathBuf };
use owo_colors::OwoColorize;

use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bulk reformat all files and produce a lookup table mapping `tn -> formatted title`.
    Format {
        /// A path to the lookup CSV file. This csv is used to rename the input files with the corresponding `tn` to the formatted title.
        #[arg(short = 'L', long)]
        lookup: String,

        /// A path to a directory containing all files to format.
        #[arg(short, long = "dir")]
        directory: String,

        /// The file extensions to include in the search.
        #[arg(short, long = "ext", default_value = "pdf")]
        extensions: Vec<String>,

        /// If true, the directory will be searched recursively.
        #[arg(short, long)]
        recursive: bool,

        /// The output directory to save the newly named files. If not provided, the formatted files will be saved in the same directory as the input files.
        /// If the directory does not exist, it will be created.
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Modify a CSV file to include volume and issue numbers for each `tn` by its formatted title.
    Populate {
        /// A path to the target CSV file to modify and populate with volume and issue numbers.
        #[arg(short = 'T', long)]
        target: String,

        /// A path to the lookup CSV file.
        #[arg(short = 'L', long)]
        lookup: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Format { lookup, directory, extensions, recursive, output } => {
            let files = collect_files(&directory, &extensions, recursive);
            let lookup_table = parse_lookup_table(&lookup);
            copy_and_rename_files(files, lookup_table, output);
        }
        Commands::Populate { target, lookup } => {
            let lookup_table = parse_lookup_table(&lookup);
            let inverse_lookup_table = lookup_table
                .into_iter()
                .map(|(_, v)| (v.formatted_title(), v))
                .collect::<BTreeMap<String, IssueData>>();
            populate_csv(&target, inverse_lookup_table).unwrap();
        }
    }

    println!("{}", "Job done.".green().bold())
}

fn populate_csv(
    target: &str,
    inverse_lookup_table: BTreeMap<String, IssueData>
) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(target).expect("Failed to read target CSV file.");
    let target = target.replace(".csv", "_populated.csv");
    let mut writer = csv::Writer::from_path(target).expect("Failed to write to target CSV file.");

    for result in reader.deserialize() {
        let mut record: Record = result?;
        if let Some(issue) = inverse_lookup_table.get(&record.node_title) {
            if let Some(volume) = issue.volume {
                record.volume = volume.to_string();
            }
            if let Some(issue) = issue.issue {
                record.issue = issue.to_string();
            }
        }
        writer.serialize(record)?;
    }

    Ok(())
}

use serde::{ Serialize, Deserialize };
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[serde(rename = "NODE_TITLE")]
    node_title: String,

    #[serde(rename = "ASSETS")]
    assets: String,

    #[serde(rename = "ATTACHMENTS")]
    attachments: String,

    #[serde(rename = "#REDACT")]
    redact: String,

    #[serde(rename = "Part Of")]
    part_of: String,

    #[serde(rename = "Previous Issue")]
    previous_issue: String,

    #[serde(rename = "Next Issue")]
    next_issue: String,

    #[serde(rename = "Creator")]
    creator: String,

    #[serde(rename = "Contributor")]
    contributor: String,

    #[serde(rename = "Publisher")]
    publisher: String,

    #[serde(rename = "Volume")]
    volume: String,

    #[serde(rename = "Issue")]
    issue: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Subject")]
    subject: String,

    #[serde(rename = "Date Original")]
    date_original: String,

    #[serde(rename = "Date Range")]
    date_range: String,

    #[serde(rename = "Type")]
    type_: String,

    #[serde(rename = "Original Format")]
    original_format: String,

    #[serde(rename = "Language")]
    language: String,

    #[serde(rename = "Contributing Institution")]
    contributing_institution: String,

    #[serde(rename = "Collection")]
    collection: String,

    #[serde(rename = "Subcollection")]
    subcollection: String,

    #[serde(rename = "Rights Statement")]
    rights_statement: String,

    #[serde(rename = "State Agency")]
    state_agency: String,

    #[serde(rename = "State Sub-Agency")]
    state_sub_agency: String,

    #[serde(rename = "Federal Legislative Branch Agency")]
    federal_legislative_branch_agency: String,

    #[serde(rename = "Federal Executive Department")]
    federal_executive_department: String,

    #[serde(rename = "Federal Executive Department Sub-Agency or Bureau")]
    federal_executive_department_sub_agency_or_bureau: String,

    #[serde(rename = "Federal Independent Agency")]
    federal_independent_agency: String,

    #[serde(rename = "Federal Board, Commission, or Committee")]
    federal_board_commission_or_committee: String,

    #[serde(rename = "Federal Quasi-Official Agency")]
    federal_quasi_official_agency: String,

    #[serde(rename = "Federal Court or Judicial Agency")]
    federal_court_or_judicial_agency: String,

    #[serde(rename = "City or Town")]
    city_or_town: String,

    #[serde(rename = "Geographic Feature")]
    geographic_feature: String,

    #[serde(rename = "Tribal Homeland")]
    tribal_homeland: String,

    #[serde(rename = "Road")]
    road: String,

    #[serde(rename = "County")]
    county: String,

    #[serde(rename = "State")]
    state: String,

    #[serde(rename = "Country")]
    country: String,

    #[serde(rename = "Agency")]
    agency: String,

    #[serde(rename = "Event")]
    event: String,

    #[serde(rename = "Oral History")]
    oral_history: String,

    #[serde(rename = "Person")]
    person: String,

    #[serde(rename = "Place")]
    place: String,

    #[serde(rename = "Topic")]
    topic: String,

    #[serde(rename = "Acquisition Note")]
    acquisition_note: String,

    #[serde(rename = "Call Number")]
    call_number: String,

    #[serde(rename = "Vertical File")]
    vertical_file: String,

    #[serde(rename = "OCLC Number")]
    oclc_number: String,

    #[serde(rename = "Date Digitized")]
    date_digitized: String,

    #[serde(rename = "Digital Format")]
    digital_format: String,

    #[serde(rename = "File Size")]
    file_size: String,

    #[serde(rename = "Digitizing Institution")]
    digitizing_institution: String,

    #[serde(rename = "Date Ingested")]
    date_ingested: String,

    #[serde(rename = "Batch Number")]
    batch_number: String,

    #[serde(rename = "Admin Notes")]
    admin_notes: String,
}

fn copy_and_rename_files(
    files: Vec<PathBuf>,
    lookup_table: BTreeMap<String, IssueData>,
    output: Option<String>
) {
    let output_dir = match output {
        Some(dir) => {
            std::fs::create_dir_all(&dir).expect("Failed to create output directory.");
            dir
        }
        None => String::new(),
    };

    for file in files {
        let file_name = file.file_name().expect("Failed to get file name.").to_string_lossy();
        // break off the extension.
        let (tn, ext) = file_name
            .split_once(".")
            .expect("Failed to split file name and extension.");

        if let Some(issue) = lookup_table.get(tn) {
            let target_file = format!("{}.{}", issue.formatted_title(), ext);
            let target_path = if output_dir.is_empty() {
                file.with_file_name(target_file)
            } else {
                PathBuf::from(output_dir.as_str()).join(target_file)
            };
            println!("Copying file \"{}\" to \"{}\"", file_name, target_path.to_string_lossy());
            std::fs::copy(file, target_path).expect("Failed to copy file.");
        }
    }
}

fn collect_files(directory: &str, extensions: &[String], recursive: bool) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in std::fs
        ::read_dir(directory)
        .expect("Failed to read directory. Path does not exist or is not a directory.") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if extensions.contains(&ext.to_string_lossy().to_string()) {
                    files.push(path.to_path_buf());
                }
            }
        } else if path.is_dir() && recursive {
            files.append(
                &mut collect_files(&path.to_string_lossy().to_string(), extensions, recursive)
            );
        }
    }
    println!("Found {} files.", files.len());

    files
}

fn parse_lookup_table(lookup: &str) -> BTreeMap<String, IssueData> {
    // ordered map
    let mut lookup_table = BTreeMap::new();

    // assert the lookup is a csv file.
    assert!(lookup.ends_with(".csv"), "Lookup table must be a CSV file.");

    let mut reader = csv::Reader::from_path(lookup).expect("Failed to read lookup table.");
    for result in reader.records() {
        let record = result.expect("Failed to parse record.");
        let tn = record.get(0).expect("Failed to get tn.");
        let title = record.get(1).expect("Failed to get title.");
        let issue_data = IssueData::new(tn.to_string(), title.to_string());
        lookup_table.insert(tn.to_string(), issue_data);
    }

    println!(
        "{} {} {}",
        "Parsed".italic().white(),
        lookup_table.len().bold().white(),
        "records from lookup table.".italic().white()
    );

    lookup_table
}

macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "[WARN]".yellow(), format_args!($($arg)*));
    };
}

macro_rules! warnok {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "[OK]".yellow().dimmed().italic(), format_args!($($arg)*).dimmed());
    };
}

#[derive(Debug)]
struct IssueData {
    tn: String,
    title: String,
    volume: Option<u32>,
    issue: Option<u32>,
    date: String,
}

impl IssueData {
    fn new(tn: String, raw_title: String) -> Self {
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
        let (title, v_n) = parts.split_once(".").expect("Failed to split title from volume/issue.");

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

                    warn!("VOLUME AND ISSUE WERE NOT COMMA SEPARATED: \"{}\"", v_n);

                    let (v, n) = v_n.split_once(" ").expect("Failed to split volume/issue.");
                    warnok!(
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
        }
    }

    fn formatted_title(&self) -> String {
        format!("{}_{}", self.title, self.date)
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
        warn!("Parts is not 3 in length. Date: \"{}\"", date);
        let mut month_i = None;
        for (i, part) in parts.iter().enumerate() {
            if month_i.is_some() {
                break;
            }
            for month in MONTHS.iter() {
                if part.starts_with(month) {
                    warnok!("Found month: \"{}\" at part {}", month, i);
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

    assert_eq!(parts.len(), 3);

    let month = parts[0].trim_end_matches('.');
    let day = parts[1].trim_end_matches(',');
    let year = parts[2];

    let month = match month {
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
    };
    let day = if day.len() == 1 { format!("0{}", day) } else { day.to_string() };

    format!("{}-{}-{}", year, month, day)
}
