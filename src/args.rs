use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

use std::fs;
use toml;
use dirs;

impl Arguments {
    // Merge the CLI arguments with the config file
    pub fn merge_with_config(mut self, config: Option<Config>) -> Self {
        if let Some(cfg) = config {
            if self.prefix.is_none() {
                self.prefix = cfg.prefix;
            }
            if self.suffix.is_none() {
                self.suffix = cfg.suffix;
            }
            if self.contain.is_none() {
                self.contain = cfg.contain;
            }
            if self.n == 1 {
                if let Some(n) = cfg.n {
                    self.n = n;
                }
            }
            if self.threads == num_cpus::get() as u32 {
                if let Some(t) = cfg.threads {
                    self.threads = t;
                }
            }
        }
        self
    }
}

impl Config {
    // Read the config file from the home directory or the current directory
    pub fn from_file() -> Option<Self> {
        let mut paths = vec![];
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join("vansol.toml"));
        }
        paths.push(PathBuf::from("vansol.toml"));
        for path in paths {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str::<Config>(&content) {
                        return Some(cfg);
                    }
                }
            }
        }
        None
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long)]
    pub prefix: Option<String>,

    #[arg(short, long)]
    pub suffix: Option<String>,

    #[arg(short, long)]
    pub contain: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    pub n: usize,

    #[arg(short, long, default_value_t = num_cpus::get() as u32)]
    pub threads: u32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub contain: Option<String>,
    pub n: Option<usize>,
    pub threads: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_threads() {
        let args = Arguments::parse_from(&[
            "vansol",
            "--prefix",
            "ahmad",
            "--suffix",
            "john",
            "--threads",
            "1",
        ]);
        assert_eq!(args.prefix, Some("ahmad".to_string()));
        assert_eq!(args.suffix, Some("john".to_string()));
        assert_eq!(args.threads, 1);
    }

    #[test]
    fn test_without_threads() {
        let args = Arguments::parse_from(&["vansol", "--prefix", "ahmad", "--suffix", "john"]);
        assert_eq!(args.prefix, Some("ahmad".to_string()));
        assert_eq!(args.suffix, Some("john".to_string()));
        assert_ne!(args.threads, 1);
    }

    #[test]
    fn test_multiple_n() {
        let args = Arguments::parse_from(&["vansol", "--prefix", "sol", "--n", "5"]);
        assert_eq!(args.prefix, Some("sol".to_string()));
        assert_eq!(args.n, 5);
    }
} 