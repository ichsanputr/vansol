use rayon::prelude::*;

use crate::args::{Arguments, Config};

use solana_sdk::{signature::Keypair, signer::Signer};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use prettytable::{Table, row};

use rand::Rng;
use chrono::Utc;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::vec::Vec;
use indicatif::ProgressBar;

pub fn generate_vanity_address(args: &Arguments, config: Option<&Config>, pb: &ProgressBar) {
    if let Some(cfg) = config {
        pb.suspend(|| {
            println!("\nLoaded config from vansol.toml:");
            let mut table = Table::new();
            table.add_row(row!["Parameter", "Value"]);
            if let Some(prefix) = &cfg.prefix {
                table.add_row(row!["prefix", prefix]);
            }
            if let Some(suffix) = &cfg.suffix {
                table.add_row(row!["suffix", suffix]);
            }
            if let Some(contain) = &cfg.contain {
                table.add_row(row!["contain", contain]);
            }
            if let Some(n) = cfg.n {
                table.add_row(row!["n", n]);
            }
            if let Some(threads) = cfg.threads {
                table.add_row(row!["threads", threads]);
            }
            table.printstd();
        });
    }

    if args.prefix.is_none() && args.suffix.is_none() && args.contain.is_none() {
        let keypair: Keypair = Keypair::new();
        let strings_bytes = serde_json::to_string_pretty(&keypair.to_bytes().to_vec()).unwrap();

        // Write the keypair to a file
        write_to_file(strings_bytes, "vanity_address.json");

        // Wait for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));

        pb.suspend(|| {
            let mut table = Table::new();

            println!("Vanity address sucessfully generated 🔥");

            table.add_row(row!["Result", "Value"]);
            table.add_row(row!["Public Key", keypair.pubkey().to_string()]);
            table.add_row(row!["Saved file", format!("vanity_address_{}.json", keypair.pubkey())]);
            table.printstd();
        });

        return;
    }

    // Set the number of threads
    // Prepare threads pool using rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads as usize)
        .build_global()
        .ok();

    // Generate the vanity address
    let (attempts, folder, time_taken) = if args.n == 1 {
        generate_single(args)
    } else {
        generate_multiple(args)
    };

    // Print the results
    pb.suspend(|| {
        println!("Vanity addresses sucessfully generated 🔥");

        let mut table = Table::new();
        table.add_row(row!["Result", "Value"]);
        table.add_row(row!["Total attempts", attempts]);
        
        if let Some(folder) = folder {
            if args.n == 1 {
                table.add_row(row!["Saved file", folder]);
            } else {
                table.add_row(row!["Saved folder", folder]);
            }
        }

        table.add_row(row!["Time taken", format!("{:.4?}", time_taken)]);
        table.printstd();
    });
}

// Generate a single vanity address
// Return the number of attempts, the saved file path, and the time taken
fn generate_single(args: &Arguments) -> (usize, Option<String>, std::time::Duration) {
    let found = AtomicBool::new(false);
    let attempts = AtomicUsize::new(0);
    let start = Instant::now();

    let mut saved_file = None;

    // Generate the vanity address
    while !found.load(Ordering::Relaxed) {
        let result = (0..1000)
            .into_par_iter()
            .filter_map(|_| {
                if found.load(Ordering::Relaxed) {
                    return None;
                }

                attempts.fetch_add(1, Ordering::Relaxed);

                let keypair: Keypair = Keypair::new();
                let pubkey = keypair.pubkey().to_string();
                let mut match_count = 0;
                let mut expected_matches = 0;

                if let Some(prefix) = &args.prefix {
                    expected_matches += 1;
                    if pubkey.starts_with(prefix) {
                        match_count += 1;
                    }
                }

                if let Some(suffix) = &args.suffix {
                    expected_matches += 1;
                    if pubkey.ends_with(suffix) {
                        match_count += 1;
                    }
                }

                if let Some(contain) = &args.contain {
                    expected_matches += 1;
                    if pubkey.contains(contain) {
                        match_count += 1;
                    }
                }

                if match_count == expected_matches && expected_matches > 0 {
                    found.store(true, Ordering::Relaxed);

                    let strings_bytes = serde_json::to_string_pretty(&keypair.to_bytes().to_vec()).unwrap();
                    let filename = format!("vanity_address_{}.json", pubkey);

                    write_to_file(strings_bytes, &filename);
                    
                    Some(filename)
                } else {
                    None
                }
            })
            .find_any(|_| true);

        if result.is_some() {
            saved_file = result;
            break;
        }
    }

    (attempts.load(Ordering::Relaxed), saved_file, start.elapsed())
}

// Generate multiple vanity addresses
// Return the number of attempts, the saved folder path, and the time taken
fn generate_multiple(args: &Arguments) -> (usize, Option<String>, std::time::Duration) {
    let attempts = AtomicUsize::new(0);
    let results = Arc::new(Mutex::new(Vec::<Keypair>::new()));
    let start = Instant::now();
    
    let mut folder_path = None;
    
    while results.lock().unwrap().len() < args.n {
        (0..5000).into_par_iter().for_each(|_| {
            if results.lock().unwrap().len() == args.n {
                return;
            }

            attempts.fetch_add(1, Ordering::Relaxed);
            
            let keypair: Keypair = Keypair::new();
            let pubkey = keypair.pubkey().to_string();
            
            let mut match_count = 0;
            let mut expected_matches = 0;
            
            if let Some(prefix) = &args.prefix {
                expected_matches += 1;
                if pubkey.starts_with(prefix) {
                    match_count += 1;
                }
            }
            
            if let Some(suffix) = &args.suffix {
                expected_matches += 1;
                if pubkey.ends_with(suffix) {
                    match_count += 1;
                }
            }
            
            if let Some(contain) = &args.contain {
                expected_matches += 1;
                if pubkey.contains(contain) {
                    match_count += 1;
                }
            }
            
            if match_count == expected_matches && expected_matches > 0 {
                results.lock().unwrap().push(keypair);
            }
        });

        // If the number of results is equal to the number of addresses to generate, save the results to a folder
        if results.lock().unwrap().len() == args.n {
            let date = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let folder = format!("vanity_addresses_{}", date);
            
            // Create the folder
            std::fs::create_dir_all(&folder).unwrap();
            
            results.lock().unwrap().par_iter().for_each(|keypair| {
                // Convert the keypair to a JSON string
                let strings_bytes =
                    serde_json::to_string_pretty(&keypair.to_bytes().to_vec()).unwrap();
                // Generate a random number
                let random_number: u16 = rand::rng().random_range(0..1000);
                let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
                let filename = format!("{}_{}", timestamp, random_number);
                write_to_folder(strings_bytes, &filename, &folder);
            });

            folder_path = Some(folder);
            break;
        }
    }

    // Return the number of attempts, the saved folder path, and the time taken
    (attempts.load(Ordering::Relaxed), folder_path, start.elapsed())
}

// Write the keypair to a file
fn write_to_file(data: String, filename: &str) {
    let mut file = File::create(filename).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

// Write the keypair to a folder
fn write_to_folder(data: String, filename: &str, folder_path: &str) {
    let mut file = File::create(format!("{}/{}.json", folder_path, filename)).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}