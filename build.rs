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
        .subcommand(clap::Command::new("company-profile").about("Get company profile/reference data for a symbol"))
        .subcommand(clap::Command::new("company-executives").about("Get key executives for a symbol"))
        .subcommand(clap::Command::new("company-peers").about("Get peer companies for a symbol"))
        .subcommand(clap::Command::new("company-financial-scores").about("Get financial scores for a symbol"))
        .subcommand(clap::Command::new("company-share-float").about("Get share float data for a symbol"))
        .subcommand(clap::Command::new("company-rating").about("Get analyst rating consensus for a symbol"))
        .subcommand(clap::Command::new("market-quote").about("Get the latest quote for a symbol"))
        .subcommand(clap::Command::new("market-historical").about("Get historical end-of-day price bars for a symbol"))
        .subcommand(clap::Command::new("market-dividends").about("Get historical dividends for a symbol"))
        .subcommand(clap::Command::new("market-splits").about("Get historical stock splits for a symbol"))
        .subcommand(clap::Command::new("market-price-change").about("Get price change percentages for a symbol"))
        .subcommand(clap::Command::new("fundamentals-income-statement").about("Get annual income statements for a symbol"))
        .subcommand(clap::Command::new("fundamentals-income-statement-as-reported").about("Get annual income statements as reported for a symbol"))
        .subcommand(clap::Command::new("fundamentals-balance-sheet").about("Get annual balance sheets for a symbol"))
        .subcommand(clap::Command::new("fundamentals-cash-flow").about("Get annual cash flow statements for a symbol"))
        .subcommand(clap::Command::new("fundamentals-ratios").about("Get annual financial ratios for a symbol"))
        .subcommand(clap::Command::new("fundamentals-metrics").about("Get annual key metrics for a symbol"))
        .subcommand(clap::Command::new("fundamentals-income-statement-growth").about("Get annual income statement growth for a symbol"))
        .subcommand(clap::Command::new("fundamentals-balance-sheet-growth").about("Get annual balance sheet growth for a symbol"))
        .subcommand(clap::Command::new("fundamentals-cash-flow-growth").about("Get annual cash flow growth for a symbol"))
        .subcommand(clap::Command::new("fundamentals-enterprise-values").about("Get annual enterprise values for a symbol"))
        .subcommand(clap::Command::new("fundamentals-analyst-estimates").about("Get annual analyst estimates for a symbol"))
        .subcommand(clap::Command::new("fundamentals-report-dates").about("Get available financial report dates for a symbol"))
        .subcommand(clap::Command::new("analyst-price-target-consensus").about("Get price target consensus for a symbol"))
        .subcommand(clap::Command::new("analyst-price-target-summary").about("Get price target summary for a symbol"))
        .subcommand(clap::Command::new("analyst-grades").about("Get analyst grade actions for a symbol"))
        .subcommand(clap::Command::new("earnings-calendar").about("Get earnings calendar rows for a date range"))
        .subcommand(clap::Command::new("treasury-rates").about("Get treasury rates for a date range"))
        .subcommand(clap::Command::new("technical-sma").about("Get simple moving average technical indicator rows for a symbol"))
        .subcommand(clap::Command::new("sec-filings").about("Get SEC filings for a symbol"))
        .subcommand(clap::Command::new("crypto-list").about("List supported cryptocurrency symbols"))
        .subcommand(clap::Command::new("crypto-quote").about("Get the latest quote for a cryptocurrency pair"))
        .subcommand(clap::Command::new("crypto-historical").about("Get historical end-of-day price bars for a cryptocurrency pair"))
        .subcommand(clap::Command::new("forex-quote").about("Get the latest quote for a forex pair"))
        .subcommand(clap::Command::new("forex-historical").about("Get historical end-of-day price bars for a forex pair"))
        .subcommand(clap::Command::new("news-stock").about("Get recent stock news for a symbol"))
        .subcommand(clap::Command::new("news-general").about("Get latest general market news"))
        .subcommand(clap::Command::new("news-articles").about("Get latest FMP articles"))
        .subcommand(clap::Command::new("news-forex").about("Get latest forex news"))
        .subcommand(clap::Command::new("news-crypto").about("Get latest crypto news"));

    let man = clap_mangen::Man::new(cmd);

    let mut buf = vec![];
    man.render(&mut buf).expect("failed to render man page");

    let out_dir = std::path::PathBuf::from("man");
    std::fs::create_dir_all(&out_dir).expect("failed to create man/ directory");

    let path = out_dir.join("fmp-agent.1");
    let mut file = std::fs::File::create(&path).expect("failed to create man page file");
    file.write_all(&buf).expect("failed to write man page");
}
