mod core;
mod args;

use indicatif::{ProgressBar, ProgressStyle};

use std::time::Duration;
use args::Arguments;
use clap::Parser;

fn main() {
    let cli_args = Arguments::parse();
    let config = args::Config::from_file();
    let args = cli_args.merge_with_config(config.clone());
    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"]),
    );
    pb.set_message("Generating vanity address...");

    // Generate the vanity address
    core::generate_vanity_address(&args, config.as_ref(), &pb);
    
    pb.finish_with_message("Done");
}

#[cfg(test)]
mod tests {
    use solana_sdk::{signature::Keypair, signature::SeedDerivable, signer::Signer};

    #[test]
    fn test_seed_derivation() {
        let keypair = Keypair::new();
        let seed = keypair.secret_bytes();
        let keypairnew = Keypair::from_seed(seed).unwrap();
        assert_eq!(keypair.pubkey(), keypairnew.pubkey());
    }

    #[test]
    fn test_bytes_validation() {
        let bytes: [u8; 64] = [
            210, 15, 55, 139, 179, 227, 210, 110, 185, 54, 116, 106, 32, 209, 48, 109, 144, 253,
            177, 226, 93, 227, 60, 87, 143, 238, 105, 188, 212, 245, 82, 227, 146, 160, 235, 241,
            249, 176, 50, 75, 217, 35, 117, 180, 161, 196, 105, 219, 66, 186, 191, 63, 185, 17,
            124, 33, 70, 96, 74, 170, 147, 64, 24, 129,
        ];
        let keypair = Keypair::try_from(&bytes[..]).unwrap();
        println!("Pubkey: {:?}", keypair.pubkey());
        assert_eq!(true, true);
    }
}
