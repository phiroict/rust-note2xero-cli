
/// Core functions we use
use noted2xero_core::n2x_core::init_logging;
use noted2xero_core::n2x_core::map_noted_to_xero;
use noted2xero_core::n2x_core::read_file;
use noted2xero_core::n2x_core::parse_noted_csv;

use noted2xero_core::xero::XeroType;

use noted2xero_core::constants;

/// Logging
use log::{info, warn, error};
extern crate glob;

/// File and date functions
use self::glob::glob;
use std::fs;
use chrono;
use chrono::Duration;
use chrono::Local;



/// Meta information
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");


/// Entrypoint
fn main() {
    // Time the action
    let start_time = Local::now();
    //Set logging
    init_logging();
    info!("Hi, I am {} at your service, this is version {}",CARGO_PKG_NAME, VERSION);
    let root = "resources/";
    let noted_folder = format!("{}{}", root, "notedfolder");
    let done_folder = format!("{}{}", root, "donefolder");
    let xero_folder = format!("{}{}", root, "xerofolder");
    process_noted_csv(&noted_folder, &done_folder, &xero_folder);

    let end_time = Local::now();
    let duration = end_time - start_time;
    info!("We're done, goodbye! This took {} seconds ", duration.to_string());
}

/// Main worker function , gets the three folders, noted in, noted done, and the xero import folder
///  # Arguments
/// * noted_folder: Existing relative folder for the noted source CSV
/// * done_folder : Existing relative folder where the processed noted CSV will end up.
/// * xero_folder: Existing folder where the generated import folder will end up.
fn process_noted_csv(noted_folder: &String, done_folder: &String, xero_folder: &String) {
    info!("noted path is {}, done path is {}, xero folder is {}", noted_folder, done_folder, xero_folder);
    let paths: glob::Paths = glob(&format!("{}/*.csv", noted_folder)[..]).unwrap();
    let mut path_counter = 0;
    for path in paths {
        if let Ok(entry) = path {
            path_counter += 1;
            let target_file = format!("{}", entry.display());
            info!("Processing file {}", target_file);
            let noted_content_result = read_file(entry.display().to_string());
            let noted_collection = parse_noted_csv(&noted_content_result.unwrap());
            let xero_collection = map_noted_to_xero(&noted_collection);
            write_xero_csv(xero_collection, xero_folder);
            // We're done, move the original file
            let copy_result = fs::copy(entry.display().to_string(),format!("{}/processed-{}.csv",done_folder.to_string(),Local::now().format("%Y-%m-%d--%s") ));
            match copy_result {
                Ok(_) => {
                    info!("Processing complete, copying file {} to location {}", entry.display().to_string(), done_folder.to_string());
                    let delete_result = fs::remove_file(entry.display().to_string());
                    match delete_result {
                        Ok(_) => {
                            info!("Noted file {} deleted", entry.display().to_string());
                        }
                        Err(err) => {
                            error!("Could not delete noted file: {} - {:?})\nYou should delete it yourself.",entry.display().to_string(), err);
                        }
                    }
                }
                Err(err) => {
                    error!("Could not copy over {}, because {:?}", entry.display().to_string(), err);
                }
            }
        }

    }
    if path_counter == 0 {
        warn!("There were no noted csvs in the noted folder, this application will leave now");
    }
}






/// Write out the xero CSV folder from the Noted collection
/// # Arguments
/// * xero_lines: collectoin of XeroTypes to get the data from.
/// * target_path: path of the directory to push the xero import file to.
fn write_xero_csv(xero_lines: Vec<XeroType>, target_path: &String) {
    let today = Local::now()  + Duration::days(constants::INVOICE_DAYS_TODAY as i64);
    let filepath = format!("{}/xero-{}.csv", target_path, today.format("%Y-%m-%d--%s"));
    let mut writer = csv::Writer::from_path(&filepath).unwrap();
    writer.write_record(XeroType::get_headers()).expect("ERR:: Could not write the headers, skipping");
    for xero_item in xero_lines.iter() {
        writer.write_record(xero_item.get_item_as_vector()).expect("ERR:: Could not save line, skipping");
    }
    let flush_result = writer.flush();
    match flush_result {
        Ok(_) => { info!("Stored Xero csv at {}",filepath) }
        Err(err) => { error!("Could not save xero file {} because: {:?}",&filepath, err) }
    }
}