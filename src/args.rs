use clap::Parser;

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
} 