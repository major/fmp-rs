#[path = "src/cli/help.rs"]
pub(crate) mod help;

mod cli {
    pub(crate) use crate::help;
}

#[path = "src/cli/args.rs"]
mod args;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/cli/args.rs");
    println!("cargo::rerun-if-changed=src/cli/help.rs");

    if std::env::var("CARGO_FEATURE_CLI").is_err() {
        return;
    }

    use clap::CommandFactory;
    use std::io::Write;

    let man = clap_mangen::Man::new(args::Cli::command());

    let mut buf = vec![];
    man.render(&mut buf).expect("failed to render man page");

    let out_dir = std::path::PathBuf::from("man");
    std::fs::create_dir_all(&out_dir).expect("failed to create man/ directory");

    let path = out_dir.join("fmp-agent.1");
    let mut file = std::fs::File::create(&path).expect("failed to create man page file");
    file.write_all(&buf).expect("failed to write man page");
}
