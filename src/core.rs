use rayon::prelude::*;

use solana_sdk::{signature::Keypair, signer::Signer};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rand::Rng;
use chrono::Utc;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::vec::Vec;
use crate::args::Arguments;

pub fn generate_vanity_address(args: &Arguments) {
    if args.prefix.is_none() && args.suffix.is_none() && args.contain.is_none() {
        println!("No arguments provided, creating a random address instead");
        let keypair: Keypair = Keypair::new();
        println!("Public Key: {:?}", keypair.pubkey());
        println!("Secret Key: {:?}", keypair.to_bytes());
        return;
    }
    let start_time = Instant::now();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads as usize)
        .build_global()
        .ok();
    if args.n == 1 {
        generate_single(args);
    } else {
        generate_multiple(args);
    }
    println!("Time taken: {:?}", start_time.elapsed());
}

fn generate_single(args: &Arguments) {
    let found = AtomicBool::new(false);
    let attempts = AtomicUsize::new(0);
    while !found.load(Ordering::Relaxed) {
        (0..1000).into_par_iter().for_each(|_| {
            if found.load(Ordering::Relaxed) {
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
                found.store(true, Ordering::Relaxed);
                println!("✅ Vanity Address Found!");
                println!("Public key: {}", pubkey);
                println!("Total attempts: {}x", attempts.load(Ordering::Relaxed));
                let strings_bytes = serde_json::to_string_pretty(&keypair.to_bytes().to_vec()).unwrap();
                let filename = format!("vanity_address_{}.json", pubkey);
                write_to_file(strings_bytes, &filename);
                println!("Saved file to: {}", filename);
            }
        });
    }
}

fn generate_multiple(args: &Arguments) {
    let attempts = AtomicUsize::new(0);
    let results = Arc::new(Mutex::new(Vec::<Keypair>::new()));
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
        if results.lock().unwrap().len() == args.n {
            println!("✅ Vanity Addresses Found!");
            println!("Total attempts: {}x", attempts.load(Ordering::Relaxed));
            let date = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let folder_path = format!("vanity_addresses_{}", date);
            std::fs::create_dir_all(&folder_path).unwrap();
            results.lock().unwrap().par_iter().for_each(|keypair| {
                let strings_bytes =
                    serde_json::to_string_pretty(&keypair.to_bytes().to_vec()).unwrap();
                let random_number: u16 = rand::rng().random_range(0..1000);
                let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
                let filename = format!("{}_{}", timestamp, random_number);
                write_to_folder(strings_bytes, &filename, &folder_path);
            });
            println!("Saved file to into folder: {}", folder_path);
            return;
        }
    }
}

fn write_to_file(data: String, filename: &str) {
    let mut file = File::create(filename).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn write_to_folder(data: String, filename: &str, folder_path: &str) {
    let mut file = File::create(format!("{}/{}.json", folder_path, filename)).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}