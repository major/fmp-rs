#[test]
fn library_dependency_example_present() {
    let readme = include_str!("../README.md");
    let crate_docs = include_str!("../src/lib.rs");
    let expected = "rusty-fmp = { default-features = false }";

    assert!(
        readme.contains(expected),
        "README library dependency example missing; expected `{expected}`"
    );
    assert!(
        crate_docs.contains(&format!("//! {expected}")),
        "crate-level library dependency example missing; expected `//! {expected}`"
    );
}
