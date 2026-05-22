fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/cli/args.rs");

    if std::env::var("CARGO_FEATURE_CLI").is_err() {
        return;
    }

    use std::io::Write;

    // Mirror the top-level Cli shape from src/cli/args.rs.
    // Only the top-level man page is generated; subcommand man pages are not.
    let cmd = clap::Command::new("fmp-agent")
        .about("Financial Modeling Prep CLI optimized for predictable JSON output.")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::new("api-key")
                .long("api-key")
                .help("FMP API key. Prefer FMP_API_KEY in .env or the environment so shells do not record it.")
                .env("FMP_API_KEY")
                .hide_env_values(true),
        )
        .arg(
            clap::Arg::new("base-url")
                .long("base-url")
                .help("FMP stable API base URL. Override for tests or proxies.")
                .env("FMP_BASE_URL")
                .default_value("https://financialmodelingprep.com/stable/"),
        )
        .arg(
            clap::Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Increase log verbosity. Use -v for INFO, -vv for DEBUG, -vvv for TRACE.")
                .action(clap::ArgAction::Count),
        )
        .subcommand(clap::Command::new("search").about("Search for a tradable symbol by ticker or company name"))
        .subcommand(clap::Command::new("company").about("Company profile, peers, executives, and scores"))
        .subcommand(clap::Command::new("market").about("Quotes, price history, distributions, and price changes"))
        .subcommand(clap::Command::new("fundamentals").about("Annual statements, ratios, metrics, growth, and estimates"))
        .subcommand(clap::Command::new("analyst").about("Analyst ratings and price target endpoints"))
        .subcommand(clap::Command::new("calendar").about("Date-based market calendars"))
        .subcommand(clap::Command::new("rates").about("Macro rates and yield data"))
        .subcommand(clap::Command::new("technical").about("Technical indicators"))
        .subcommand(clap::Command::new("filings").about("SEC filing lookups"))
        .subcommand(clap::Command::new("crypto").about("Cryptocurrency quotes and historical prices"))
        .subcommand(clap::Command::new("forex").about("Forex quotes and historical prices"))
        .subcommand(clap::Command::new("news").about("News endpoints"));

    let man = clap_mangen::Man::new(cmd);

    let mut buf = vec![];
    man.render(&mut buf).expect("failed to render man page");

    let out_dir = std::path::PathBuf::from("man");
    std::fs::create_dir_all(&out_dir).expect("failed to create man/ directory");

    let path = out_dir.join("fmp-agent.1");
    let mut file = std::fs::File::create(&path).expect("failed to create man page file");
    file.write_all(&buf).expect("failed to write man page");
}
