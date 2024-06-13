use serde::{ Serialize, Deserialize };
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveRecord {
    #[serde(rename = "NODE_TITLE")]
    pub node_title: String,

    #[serde(rename = "ASSETS")]
    pub assets: String,

    #[serde(rename = "ATTACHMENTS")]
    pub attachments: String,

    #[serde(rename = "#REDACT")]
    pub redact: String,

    #[serde(rename = "Part Of")]
    pub part_of: String,

    #[serde(rename = "Previous Issue")]
    pub previous_issue: String,

    #[serde(rename = "Next Issue")]
    pub next_issue: String,

    #[serde(rename = "Creator")]
    pub creator: String,

    #[serde(rename = "Contributor")]
    pub contributor: String,

    #[serde(rename = "Publisher")]
    pub publisher: String,

    #[serde(rename = "Volume")]
    pub volume: String,

    #[serde(rename = "Issue")]
    pub issue: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Subject")]
    pub subject: String,

    #[serde(rename = "Date Original")]
    pub date_original: String,

    #[serde(rename = "Date Range")]
    pub date_range: String,

    #[serde(rename = "Type")]
    pub type_: String,

    #[serde(rename = "Original Format")]
    pub original_format: String,

    #[serde(rename = "Language")]
    pub language: String,

    #[serde(rename = "Contributing Institution")]
    pub contributing_institution: String,

    #[serde(rename = "Collection")]
    pub collection: String,

    #[serde(rename = "Subcollection")]
    pub subcollection: String,

    #[serde(rename = "Rights Statement")]
    pub rights_statement: String,

    #[serde(rename = "State Agency")]
    pub state_agency: String,

    #[serde(rename = "State Sub-Agency")]
    pub state_sub_agency: String,

    #[serde(rename = "Federal Legislative Branch Agency")]
    pub federal_legislative_branch_agency: String,

    #[serde(rename = "Federal Executive Department")]
    pub federal_executive_department: String,

    #[serde(rename = "Federal Executive Department Sub-Agency or Bureau")]
    pub federal_executive_department_sub_agency_or_bureau: String,

    #[serde(rename = "Federal Independent Agency")]
    pub federal_independent_agency: String,

    #[serde(rename = "Federal Board, Commission, or Committee")]
    pub federal_board_commission_or_committee: String,

    #[serde(rename = "Federal Quasi-Official Agency")]
    pub federal_quasi_official_agency: String,

    #[serde(rename = "Federal Court or Judicial Agency")]
    pub federal_court_or_judicial_agency: String,

    #[serde(rename = "City or Town")]
    pub city_or_town: String,

    #[serde(rename = "Geographic Feature")]
    pub geographic_feature: String,

    #[serde(rename = "Tribal Homeland")]
    pub tribal_homeland: String,

    #[serde(rename = "Road")]
    pub road: String,

    #[serde(rename = "County")]
    pub county: String,

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Country")]
    pub country: String,

    #[serde(rename = "Agency")]
    pub agency: String,

    #[serde(rename = "Event")]
    pub event: String,

    #[serde(rename = "Oral History")]
    pub oral_history: String,

    #[serde(rename = "Person")]
    pub person: String,

    #[serde(rename = "Place")]
    pub place: String,

    #[serde(rename = "Topic")]
    pub topic: String,

    #[serde(rename = "Acquisition Note")]
    pub acquisition_note: String,

    #[serde(rename = "Call Number")]
    pub call_number: String,

    #[serde(rename = "Vertical File")]
    pub vertical_file: String,

    #[serde(rename = "OCLC Number")]
    pub oclc_number: String,

    #[serde(rename = "Date Digitized")]
    pub date_digitized: String,

    #[serde(rename = "Digital Format")]
    pub digital_format: String,

    #[serde(rename = "File Size")]
    pub file_size: String,

    #[serde(rename = "Digitizing Institution")]
    pub digitizing_institution: String,

    #[serde(rename = "Date Ingested")]
    pub date_ingested: String,

    #[serde(rename = "Batch Number")]
    pub batch_number: String,

    #[serde(rename = "Admin Notes")]
    pub admin_notes: String,
}
