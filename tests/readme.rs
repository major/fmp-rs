#[test]
fn library_dependency_example_uses_current_package_version() {
    let readme = include_str!("../README.md");
    let crate_docs = include_str!("../src/lib.rs");
    let version = env!("CARGO_PKG_VERSION");
    let expected = format!("rusty-fmp = {{ version = \"{version}\", default-features = false }}");

    assert!(
        readme.contains(&expected),
        "README library dependency example must match Cargo.toml package.version; expected to find `{expected}`"
    );
    assert!(
        crate_docs.contains(&format!("//! {expected}")),
        "crate-level library dependency example must match Cargo.toml package.version; expected to find `//! {expected}`"
    );
}
