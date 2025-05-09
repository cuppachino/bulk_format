use std::{ collections::BTreeMap, path::PathBuf };
use bulk_format::{ prompt_bool, safely_target_file };
use owo_colors::OwoColorize;
use clap::{ Parser, Subcommand };

mod archive_record;
mod issue_data;
mod date;

use issue_data::IssueData;
use date::Date;

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
        #[arg(short, long)]
        target: String,

        /// A path to the lookup CSV file.
        #[arg(short = 'L', long)]
        lookup: String,
    },

    /// Populate a CSV file with `previous` and `next` issue data, using the order of the records and their node titles.
    LinkIssues {
        /// A path to the target CSV file to modify and populate with `previous` and `next` issue data.
        #[arg(short, long)]
        target: String,
    },

    /// Compare a lookup table with a generated lookup table and identify missing entries.
    Compare {
        /// A path to the lookup CSV file.
        #[arg(short = 'L', long)]
        lookup: String,

        /// A path to the generated lookup CSV file.
        #[arg(short, long)]
        generated: String,
    },

    /// Group files into directories where each directory contains at most `n` files.
    GroupFiles {
        /// A path to the directory containing all files to group.
        #[arg(short, long = "dir")]
        directory: String,

        /// The file extensions to include in the search.
        #[arg(short, long = "ext", default_value = "pdf")]
        extensions: Vec<String>,

        /// If true, the directory will be searched recursively.
        #[arg(short, long)]
        recursive: bool,

        /// The number of files to include in each group. If the number of files in the directory is not divisible by `n`, the last group will contain the remainder.
        #[arg(short)]
        n: usize,
    },
}

macro_rules! print_warn {
    ($($arg:tt)*) => {
            eprintln!("{} {}", "[WARN]".yellow(), format_args!($($arg)*));
    };
}
pub(crate) use print_warn;

macro_rules! print_warn_ok {
    ($($arg:tt)*) => {
            eprintln!("{} {}", "[OK]".yellow().dimmed().italic(), format_args!($($arg)*).dimmed());
    };
}
pub(crate) use print_warn_ok;

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
                .map(|(_, v)| (v.record_title(), v))
                .collect::<BTreeMap<String, IssueData>>();
            populate_csv(&target, inverse_lookup_table).unwrap();
        }
        Commands::LinkIssues { target } => {
            link_issues(&target);
        }
        Commands::Compare { lookup, generated } => {
            let lookup_table = parse_lookup_table(&lookup);
            let generated_names = parse_generated_names(&generated);
            compare_tables(lookup_table, generated_names);
        }
        Commands::GroupFiles { directory, extensions, recursive, n } => {
            group_files(&directory, &extensions, recursive, n);
        }
    }

    println!("{}", "Job done.".green().bold())
}

fn populate_csv(
    target: &str,
    inverse_lookup_table: BTreeMap<String, IssueData>
) -> Result<(), csv::Error> {
    use archive_record::ArchiveRecord;

    let mut reader = csv::Reader::from_path(target).expect("Failed to read target CSV file.");
    let target = target.replace(".csv", "_populated.csv");

    // if the target file already exists, prompt the user if they want to overwrite it.
    if std::path::Path::new(&target).exists() {
        let should_overwrite = prompt_bool(
            &format!("The target file \"{}\" already exists. Do you want to overwrite it?", target)
        );
        if !should_overwrite {
            print_warn_ok!("Exiting without overwriting target file.");
            return Ok(());
        }
    }

    let mut writer = csv::Writer::from_path(target).expect("Failed to write to target CSV file.");

    for result in reader.deserialize() {
        let mut record: ArchiveRecord = result?;
        if let Some(issue) = inverse_lookup_table.get(&record.node_title) {
            record.date_digitized = issue.date_loaded.to_string();
            if let Some(volume) = issue.volume {
                record.volume = volume.to_string();
            }
            if let Some(issue) = issue.issue {
                record.issue = issue.to_string();
            }
        } else {
            print_warn!("Failed to find issue data for \"{}\".", record.node_title);
        }
        writer.serialize(record)?;
    }

    Ok(())
}

/// Populate a CSV file with `previous` and `next` issue data, using the order of the records and their node titles.
fn link_issues(target: &str) {
    use archive_record::ArchiveRecord;

    let mut reader = csv::Reader::from_path(target).expect("Failed to read target CSV file.");
    let target = safely_target_file(&target.replace(".csv", "_linked.csv"));

    let mut writer = csv::Writer
        ::from_path(target.clone())
        .expect("Failed to write to target CSV file.");

    let mut records: Vec<ArchiveRecord> = reader
        .deserialize()
        .map(|r| r.expect("Failed to parse record."))
        .collect();
    let og_records = records.clone();

    for (i, record) in records.iter_mut().enumerate() {
        if i > 0 {
            record.previous_issue = og_records[i - 1].node_title.clone();
        }
        if i < og_records.len() - 1 {
            record.next_issue = og_records[i + 1].node_title.clone();
        }
        writer.serialize(record).expect("Failed to write record.");
    }

    println!("Linked issues and saved to \"{}\".", target);
}

fn group_files(directory: &str, extensions: &[String], recursive: bool, n: usize) {
    let files = collect_files(directory, extensions, recursive);
    let groups = files.chunks(n);

    for (i, group) in groups.enumerate() {
        // if the files have dates at the end, find the min and max dates.
        let mut dates: Vec<Date> = vec![];
        for file in group {
            // split on the last underscore, everything after is the date.
            if
                let Some(date) = file
                    .file_name()
                    .expect("Failed to get file name.")
                    .to_string_lossy()
                    .split("_")
                    .last()
            {
                let date = date.split(".").next().expect("Failed to split date.");
                let date = Date::try_from(date).expect("Failed to parse date.");
                dates.push(date);
            }
        }

        #[allow(unused_parens)]
        let group_dir = if
            let Some((min_date, max_date)) = ({
                dates
                    .iter()
                    .min()
                    .map(|min_date| {
                        dates
                            .iter()
                            .max()
                            .map(|max_date| (min_date, max_date))
                    })
                    .flatten()
            })
        {
            let min_date = min_date.year;
            let max_date = max_date.year;
            format!("{}/{i}_{min_date}-{max_date}", directory)
        } else {
            format!("{}/{i}", directory)
        };
        std::fs::create_dir_all(&group_dir).expect("Failed to create group directory.");

        for file in group {
            let target = PathBuf::from(group_dir.as_str()).join(
                file.file_name().expect("Failed to get file name.")
            );
            println!(
                "Moving file \"{}\" to \"{}\"",
                file.to_string_lossy(),
                target.to_string_lossy()
            );
            std::fs::rename(file, target).expect("Failed to move file.");
        }
    }
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

    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

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
        let date_loaded = record.get(5).expect("Failed to get date loaded.");

        if tn.is_empty() {
            continue;
        }

        let issue_data = IssueData::new(tn.to_string(), title.to_string(), date_loaded.to_string());
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

fn parse_generated_names(generated: &str) -> Vec<String> {
    let mut names = vec![];

    // assert the lookup is a csv file.
    assert!(generated.ends_with(".csv"), "Generated table must be a CSV file.");

    let mut reader = csv::Reader::from_path(generated).expect("Failed to read generated table.");
    for result in reader.records() {
        let record = result.expect("Failed to parse record.");
        // Arizona Catering Employees, 1944-05-12
        let node_title = record.get(0).expect("Failed to get node title.");
        // // split on comma, extract the date.
        // let date = node_title.split(", ").last().expect("Failed to split date from node title.");
        names.push(node_title.to_string());
    }

    println!(
        "{} {} {}",
        "Parsed".italic().white(),
        names.len().bold().white(),
        "records from generated table.".italic().white()
    );

    names
}

fn compare_tables(lookup_table: BTreeMap<String, IssueData>, generated_names: Vec<String>) {
    // check if any of the dates in the lookup table are the same.
    {
        let mut duplicate_dates = vec![];
        for (tn, issue) in &lookup_table {
            if
                lookup_table
                    .values()
                    .filter(|i| i.date == issue.date)
                    .count() > 1
            {
                duplicate_dates.push((tn, issue.date.clone()));
            }
        }
        if duplicate_dates.is_empty() {
            println!("{}", "No duplicate dates found.".green().bold());
        } else {
            println!("{}", "Duplicate dates:".red().bold());
            for (tn, date) in duplicate_dates {
                println!("{}: {}", tn, date);
            }
        }
    }

    let mut missing = vec![];
    let mut indexes = vec![];
    // for each name in the lookup table, check if it exists in the generated names, and if it does not, add it to the missing list.
    for (tn, issue) in lookup_table {
        if !generated_names.contains(&issue.record_title()) {
            missing.push((tn, issue.record_title()));
        } else {
            // get the index of the generated name.
            let index = generated_names
                .iter()
                .position(|n| n.contains(&issue.record_title()))
                .unwrap();
            indexes.push(index);
        }
    }

    // sort the indexes, verify they are sequential.
    indexes.sort();
    let mut last = -1;
    let mut is_sequential = true;
    for index in indexes.iter() {
        let index = index.clone() as i32;
        if index != last + 1 {
            print_warn!("Index {} is not sequential.", index);
            is_sequential = false;
        }
        last = index;
    }
    if is_sequential {
        println!(
            "{} {}..{}",
            "Indexes are sequential.".green().bold(),
            indexes[0],
            indexes.last().unwrap()
        );
    } else {
        println!("{}", "Indexes are not sequential.".red().bold());
    }
    println!("{} Total verified files.", indexes.len());

    if missing.is_empty() {
        println!("{}", "No missing entries found.".green().bold());
    } else {
        println!("{}", "Missing entries:".red().bold());
        for (tn, title) in missing {
            println!("{}: {}", tn, title);
        }
    }
}
